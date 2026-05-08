use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::error::{AppError, AppResult};

const RATHOLE_REPO: &str = "rathole-org/rathole";
const GITHUB_API: &str = "https://api.github.com";
const USER_AGENT: &str = concat!("rathole-launcher/", env!("CARGO_PKG_VERSION"));
const CHECK_TIMEOUT_SECS: u64 = 10;
const DOWNLOAD_TIMEOUT_SECS: u64 = 300;

#[derive(Deserialize, Debug)]
struct GhRelease {
    tag_name: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    published_at: String,
    assets: Vec<GhAsset>,
}

#[derive(Deserialize, Debug)]
struct GhAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

#[derive(Serialize, Clone, Debug)]
pub struct AssetInfo {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub target: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct UpdateCheckResult {
    pub installed_version: Option<String>,
    pub latest_version: Option<String>,
    pub release_name: Option<String>,
    pub published_at: Option<String>,
    pub asset: Option<AssetInfo>,
    pub update_available: bool,
    pub binary_present: bool,
    pub github_reachable: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: Option<u64>,
}

/// Returns the rathole release target triple matching the host.
pub fn current_target_triple() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        ("linux", "x86_64") => Some("x86_64-unknown-linux-musl"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-musl"),
        ("linux", "arm") => Some("armv7-unknown-linux-musleabihf"),
        _ => None,
    }
}

fn build_client(timeout_secs: u64) -> AppResult<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| AppError::Other(format!("http client build failed: {e}")))
}

fn pick_asset(release: &GhRelease) -> Option<AssetInfo> {
    let target = current_target_triple()?;

    let exact = release
        .assets
        .iter()
        .find(|a| a.name.contains(target) && a.name.ends_with(".zip"));
    if let Some(a) = exact {
        return Some(AssetInfo {
            name: a.name.clone(),
            url: a.browser_download_url.clone(),
            size: a.size,
            target: target.to_string(),
        });
    }

    // macOS Apple Silicon: fall back to the Intel build, which runs fine
    // under Rosetta 2 if the native aarch64 binary isn't published yet.
    if std::env::consts::OS == "macos" && std::env::consts::ARCH == "aarch64" {
        let fallback_target = "x86_64-apple-darwin";
        if let Some(a) = release
            .assets
            .iter()
            .find(|a| a.name.contains(fallback_target) && a.name.ends_with(".zip"))
        {
            return Some(AssetInfo {
                name: a.name.clone(),
                url: a.browser_download_url.clone(),
                size: a.size,
                target: format!("{fallback_target} (via Rosetta 2)"),
            });
        }
    }

    None
}

async fn detect_installed(rathole_path: &Path) -> Option<String> {
    if !rathole_path.exists() {
        return None;
    }
    let output = tokio::process::Command::new(rathole_path)
        .arg("--version")
        .output()
        .await
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = if stdout.trim().is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        stdout.to_string()
    };
    combined.split_whitespace().find_map(extract_semver)
}

/// Parses a `[v]MAJOR.MINOR.PATCH` from a single token, stripping any
/// `git describe` suffix (e.g. `v0.5.0-3-g3a73a0c` → `0.5.0`). Returns
/// `None` for tokens that don't look like a semver — this lets the caller
/// scan whitespace-separated output from `rathole --version`, which
/// includes a build timestamp and a git-describe string, and pick the
/// version regardless of which position it occupies.
pub fn extract_semver(token: &str) -> Option<String> {
    let trimmed = token.trim_start_matches('v');
    let head = trimmed.split('-').next()?;
    let parts: Vec<&str> = head.split('.').collect();
    if parts.len() < 3 {
        return None;
    }
    if parts
        .iter()
        .take(3)
        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
    {
        Some(parts[..3].join("."))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::extract_semver;

    #[test]
    fn parses_clean_tag() {
        assert_eq!(extract_semver("v0.5.0").as_deref(), Some("0.5.0"));
        assert_eq!(extract_semver("0.5.0").as_deref(), Some("0.5.0"));
    }

    #[test]
    fn strips_git_describe_suffix() {
        assert_eq!(
            extract_semver("v0.5.0-3-g3a73a0c").as_deref(),
            Some("0.5.0"),
        );
    }

    #[test]
    fn rejects_non_semver_tokens() {
        assert_eq!(extract_semver("rathole"), None);
        assert_eq!(extract_semver("Build"), None);
        assert_eq!(extract_semver("2023-09-14T17:51:32.797893000Z"), None);
    }
}

pub async fn check_update(rathole_path: PathBuf) -> UpdateCheckResult {
    let installed = detect_installed(&rathole_path).await;
    let binary_present = rathole_path.exists();

    let client = match build_client(CHECK_TIMEOUT_SECS) {
        Ok(c) => c,
        Err(e) => {
            return UpdateCheckResult {
                installed_version: installed,
                latest_version: None,
                release_name: None,
                published_at: None,
                asset: None,
                update_available: false,
                binary_present,
                github_reachable: false,
                error: Some(e.to_string()),
            };
        }
    };

    let url = format!("{GITHUB_API}/repos/{RATHOLE_REPO}/releases/latest");
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return UpdateCheckResult {
                installed_version: installed,
                latest_version: None,
                release_name: None,
                published_at: None,
                asset: None,
                update_available: false,
                binary_present,
                github_reachable: false,
                error: Some(e.to_string()),
            };
        }
    };

    if !response.status().is_success() {
        return UpdateCheckResult {
            installed_version: installed,
            latest_version: None,
            release_name: None,
            published_at: None,
            asset: None,
            update_available: false,
            binary_present,
            github_reachable: true,
            error: Some(format!("github responded with status {}", response.status())),
        };
    }

    let release: GhRelease = match response.json().await {
        Ok(r) => r,
        Err(e) => {
            return UpdateCheckResult {
                installed_version: installed,
                latest_version: None,
                release_name: None,
                published_at: None,
                asset: None,
                update_available: false,
                binary_present,
                github_reachable: true,
                error: Some(format!("decode release: {e}")),
            };
        }
    };

    let latest_version = release.tag_name.trim_start_matches('v').to_string();
    let asset = pick_asset(&release);

    // Treat "binary file present but `--version` failed" as a separate
    // state from "no binary at all". The former usually means the OS
    // (Windows Smart App Control / Defender / AppLocker) is blocking
    // the executable; offering another download would just re-trigger
    // the same block in a loop. The frontend renders a dedicated
    // banner for that case instead of the update offer.
    let update_available = match (&installed, &asset) {
        (Some(inst), Some(_)) => inst != &latest_version,
        (None, Some(_)) if !binary_present => true,
        _ => false,
    };

    UpdateCheckResult {
        installed_version: installed,
        latest_version: Some(latest_version),
        release_name: Some(release.name),
        published_at: Some(release.published_at),
        asset,
        update_available,
        binary_present,
        github_reachable: true,
        error: None,
    }
}

pub async fn download_and_install(
    app: &AppHandle,
    url: &str,
    target_path: &Path,
) -> AppResult<()> {
    let parent = target_path
        .parent()
        .ok_or_else(|| AppError::Other("target path has no parent".into()))?;
    std::fs::create_dir_all(parent)?;

    let archive_path = parent.join(".rathole-download.zip");
    let _ = std::fs::remove_file(&archive_path);

    let client = build_client(DOWNLOAD_TIMEOUT_SECS)?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::Other(format!("download request failed: {e}")))?;

    if !response.status().is_success() {
        return Err(AppError::Other(format!(
            "download failed with status {}",
            response.status()
        )));
    }

    let total = response.content_length();
    let _ = app.emit(
        "rathole-download-progress",
        DownloadProgress { downloaded: 0, total },
    );

    {
        let mut file = std::fs::File::create(&archive_path)?;
        let mut stream = response.bytes_stream();
        let mut downloaded: u64 = 0;
        let mut last_emit_at = std::time::Instant::now();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| AppError::Other(format!("download stream: {e}")))?;
            std::io::Write::write_all(&mut file, &chunk)?;
            downloaded += chunk.len() as u64;
            let now = std::time::Instant::now();
            if now.duration_since(last_emit_at).as_millis() >= 100 {
                let _ = app.emit(
                    "rathole-download-progress",
                    DownloadProgress { downloaded, total },
                );
                last_emit_at = now;
            }
        }

        let _ = app.emit(
            "rathole-download-progress",
            DownloadProgress {
                downloaded,
                total: Some(downloaded),
            },
        );
    }

    extract_rathole(&archive_path, target_path)?;
    let _ = std::fs::remove_file(&archive_path);
    Ok(())
}

fn extract_rathole(zip_path: &Path, dest: &Path) -> AppResult<()> {
    let target_name = if cfg!(windows) { "rathole.exe" } else { "rathole" };

    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::Other(format!("open zip: {e}")))?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| AppError::Other(format!("read zip entry: {e}")))?;
        let entry_name = entry.name().to_string();
        let basename = entry_name.rsplit('/').next().unwrap_or(&entry_name);
        if basename == target_name {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = std::fs::File::create(dest)?;
            std::io::copy(&mut entry, &mut out)?;
            drop(out);

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(dest)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(dest, perms)?;
            }

            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("xattr")
                    .args(["-d", "com.apple.quarantine"])
                    .arg(dest)
                    .output();
            }

            return Ok(());
        }
    }
    Err(AppError::Other(
        "rathole binary not found inside the downloaded archive".into(),
    ))
}
