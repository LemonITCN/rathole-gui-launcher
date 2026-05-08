<div align="center">

# Rathole Launcher

[rathole](https://github.com/rathole-org/rathole) 리버스 프록시 터널을 몇 번의 클릭으로 다룰 수 있게 해주는 데스크톱 앱 — macOS, Windows, Linux 지원.

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md) · **한국어**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](../../releases)
[![Tauri](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=white)](https://tauri.app)
[![Vue 3](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-stable-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org)

[다운로드](../../releases) · [주요 기능](#주요-기능) · [빠른 시작](#빠른-시작) · [소스에서 빌드](#소스에서-빌드)

</div>

<p align="center">
  <img src="docs/screenshots/launcher-client.png" alt="Rathole Launcher 메인 창" width="780" />
</p>

---

## 왜 만들었나

rathole을 직접 구성해 본 분이라면 익숙하실 겁니다. 서버에 SSH 접속, TOML 두 개 편집, 토큰을 옮겨 적기, 데몬 재시작, stderr 따라가며 글자 하나 틀리지 않았기를 빌기. rathole 자체는 정말 훌륭합니다 — 다만 설정 의식이 조금 번거로울 뿐입니다.

Rathole Launcher는 그 번거로움을 없애기 위해 존재합니다. 모든 옵션이 폼 필드로 대응되고, 토큰은 서버 설정에서 자동으로 채워지며, 로그는 실시간으로 흐르고, 시작·중지·재시작은 클릭 한 번이면 됩니다. Launcher 자체가 죽어도 터널은 계속 돌고, 다음 실행 때 자연스럽게 다시 연결됩니다.

처음 rathole을 만났을 때 있었으면 했던 도구를 만들었습니다. 같이 써주시면 좋겠습니다.

---

## 주요 기능

### 🪟 rathole 전체 사양을 다루는 비주얼 에디터

문서에 있는 모든 옵션을 친근한 폼으로 설정할 수 있습니다. 주소, 토큰, 하트비트, 재시도, 서비스별 오버라이드, 그리고 전체 전송 계층(TCP / TLS / Noise / WebSocket)과 각각의 하위 필드까지. 고급 설정은 기본적으로 접혀 있어, 단순한 구성은 끝까지 단순하게 유지됩니다.

### 🚀 시작·중지·관찰·재시작 — 모두 한 창에서

각 에디터 상단에는 화면에 고정된 컨트롤 바가 있어 시작·중지·저장·재시작 버튼이 항상 보입니다. 로그는 그 아래에서 실시간으로 흐르며, 스트림별로 색상이 구분되고, 인스턴스마다 1000줄 링 버퍼에 저장됩니다. 터널이 동작하는 중 설정을 변경하면, 노란색 줄이 나타나 "지금 재시작"을 한 번에 적용해 줍니다.

### 🤖 자체 rathole 사용 — 또는 우리에게 맡기기

Launcher 옆에 rathole 바이너리가 없으면, `rathole-org/rathole`의 최신 릴리스에서 OS와 CPU에 맞는 아카이브를 다운로드하자는 배너가 앱 안에 나타납니다. Apple Silicon에서 네이티브 ARM 빌드가 아직 공개되지 않았다면, Intel 빌드로 자동 폴백되어 Rosetta 2로 실행됩니다. 새 릴리스가 있는지도 매 실행 시 확인하고 같은 자리에서 알려드립니다.

### 📋 서버의 `server.toml`을 붙여넣으면 클라이언트 설정 완성

사이드바의 **+** 클릭 → **서버 TOML 가져오기**로 전환 → 서버 파일 붙여넣기 → **분석** 클릭. 서비스 이름, 토큰, 전송 계층 설정이 자동으로 추출되니, 각 원격 서비스를 로컬 주소로 매핑만 하시면 됩니다.

### 🛟 자체 크래시에도 강함

Launcher가 시작한 모든 rathole PID는 디스크에 영속화됩니다. 정전, OOM, 실수로 친 `kill -9`로 Launcher가 죽어도 rathole 자식 프로세스는 계속 터널링합니다(launchd / init / systemd로 재부모화됩니다). Launcher가 다시 시작되면 고아가 된 프로세스를 발견하고 다시 연결해, 다시 관리할 수 있게 해줍니다.

### 🍴 macOS·Windows·Linux 모두 트레이 상주

창을 닫아도 앱은 종료되지 않고, macOS 메뉴 바, Windows 시스템 트레이, Linux StatusNotifier에 들어갑니다. 거기서 어떤 구성이든 열거나 시작·중지할 수 있고, 실행 중인 개수를 한눈에 볼 수 있고, 정말로 종료할 수도 있습니다 — 종료 시에는 모든 터널을 우아하게 셧다운한 뒤 앱이 끝납니다.

### 🌍 4개 언어 지원

UI **그리고** 트레이 메뉴가 **简体中文, English, 日本語, 한국어**로 번역되어 있습니다. 첫 실행 시 시스템 로케일에서 자동 감지되고, 실행 간 기억되며, 좌측 하단의 언어 선택기에서 재시작 없이 언제든 전환할 수 있습니다.

### 🤝 다른 인스턴스와 공존

Launcher는 자기가 시작한 프로세스만 건드립니다. 머신에서 돌고 있는 다른 rathole은 그대로 두고, 「설정」페이지에 "외부" 라벨로 표시합니다. 이미 사용 중인 포트로 서버 프로필을 시작하려고 하면, Launcher는 거부하고 어떤 주소가 충돌했는지 정확히 알려드립니다.

---

## 빠른 시작

### 1. 설치

[GitHub Release](../../releases)마다 빌드된 번들이 첨부되어 있습니다.

| 플랫폼 | 받기 | 실행 |
| --- | --- | --- |
| **macOS**(Apple Silicon / Intel) | `*.dmg` | `Applications`로 드래그, 첫 실행 시 우클릭 → **열기** |
| **Windows**(x64) | `*.msi` 또는 `*-setup.exe` | 설치 프로그램 실행 |
| **Linux**(x64) | `*.AppImage` / `*.deb` / `*.rpm` | `chmod +x` 후 실행, 또는 패키지 매니저로 설치 |

번들은 기본적으로 서명되지 않은 상태이므로, 첫 실행 시 OS가 경고를 표시합니다 — 정상입니다. 본인 인증서로 서명하면 이 단계를 건너뛸 수 있습니다.

### 2. rathole 바이너리 준비

첫 실행 시 Launcher 옆에 `rathole`이 없으면 다음 배너가 보입니다:

> **rathole 실행 파일을 찾을 수 없음**
> GitHub에서 rathole 0.5.0을 자동으로 다운로드하거나, 설정에서 경로를 지정할 수 있습니다.
> &nbsp; &nbsp; **[ 다운로드 ]** &nbsp; **[ 설정 열기 ]**

**다운로드** 클릭하고 몇 초 기다리면 끝입니다. 본인 바이너리를 쓰고 싶다면 Launcher와 같은 폴더에 두거나, **설정**에서 임의의 경로를 지정하세요.

### 3. 구성 만들기

두 가지 길이 있습니다. 편한 쪽을 고르세요:

**직접 입력** — 사이드바의 **+** 클릭, 폼 채우기. 각 구성은 Launcher 옆의 `server_conf/` 또는 `client_conf/`에 일반 TOML로 저장되니, 나중에 직접 수정하거나 버전 관리에 넣을 수 있습니다.

**서버 TOML 가져오기** *(클라이언트 권장)* — **+** 클릭 → **서버 TOML 가져오기**로 전환 → 서버 파일 붙여넣기 → **분석** 클릭. 각 원격 서비스를 로컬 주소(예: `127.0.0.1:22`)로 매핑하고 **생성** 클릭. 완성된 클라이언트 구성이 만들어집니다.

### 4. 실행

**시작** 클릭. 로그가 흐르기 시작합니다. 모든 게 잘 되어 있으면 1~2초 안에 서비스가 서버에 등록되는 모습을 볼 수 있습니다. 포트가 이미 사용 중이거나 토큰이 한 글자 다르면, 어디를 고쳐야 할지 곧바로 알 수 있습니다.

---

## 소스에서 빌드

### 사전 준비

- **Node.js** ≥ 18
- **Rust** stable ≥ 1.77 — [rustup](https://rustup.rs)으로 설치
- **Tauri 2 플랫폼 의존성** — [공식 가이드](https://tauri.app/start/prerequisites/) 참조 (macOS는 Xcode CLT, Windows는 MSVC 빌드 도구, Linux는 GTK + WebKit2GTK 4.1 + libsoup-3 + libappindicator)

### 개발

```sh
git clone https://github.com/<owner>/rathole-gui-launcher.git
cd rathole-gui-launcher
npm install
npm run tauri:dev
```

첫 컴파일은 약 600개의 Rust crate를 가져오며 머신에 따라 5~15분 정도 걸립니다. 이후 실행은 증분 컴파일이라 빠릅니다.

### 로컬에서 번들링

`scripts/` 폴더에 플랫폼별 원샷 빌더가 들어 있습니다. 각 스크립트는 툴체인을 확인하고, 의존성을 설치하고, `tauri build`를 실행한 뒤 최종 번들 경로를 출력합니다.

| 플랫폼 | 명령 |
| --- | --- |
| **macOS** | `./scripts/build-macos.sh` &nbsp; *(옵션: `--silicon`, `--intel`, `--universal`)* |
| **Linux** | `./scripts/build-linux.sh` |
| **Windows** | `pwsh scripts/build-windows.ps1` |

### GitHub Actions으로 릴리스

`.github/workflows/release.yml`은 `v*` 태그가 push되면 **macOS Apple Silicon, macOS Intel, Windows x64, Linux x64** 빌드를 병렬로 수행하고, 모든 결과물을 드래프트 GitHub Release에 첨부합니다:

```sh
# src-tauri/tauri.conf.json의 버전을 올리고 커밋한 뒤:
git tag v0.1.0
git push origin v0.1.0
```

`APPLE_CERTIFICATE`, `TAURI_SIGNING_PRIVATE_KEY` 등 리포지토리 secret이 설정되어 있으면 Apple / Windows 코드 서명이 자동으로 적용됩니다. 없으면 서명되지 않은 번들이 생성됩니다 — 초기 릴리스에는 충분합니다.

---

## 기술 스택

| 계층 | 도구 |
| --- | --- |
| 윈도우 셸 | [Tauri 2](https://tauri.app) |
| 프론트엔드 | [Vue 3](https://vuejs.org) `<script setup>`, [Vue Router](https://router.vuejs.org), [Pinia](https://pinia.vuejs.org), [Vue I18n](https://vue-i18n.intlify.dev) |
| UI 라이브러리 | [Antdv Next](https://github.com/antdv-next/antdv-next) — Ant Design Vue의 현대 Vue 3 재작성 |
| 백엔드 | Rust + Tokio · [reqwest](https://crates.io/crates/reqwest)(rustls) · [sysinfo](https://crates.io/crates/sysinfo) · [zip](https://crates.io/crates/zip) · [toml](https://crates.io/crates/toml)(순서 보존) · [nix](https://crates.io/crates/nix) |
| 빌드 | Vite, vue-tsc, tauri-cli |

---

## 기여하기

이슈와 PR 환영합니다. 매끄럽게 진행하기 위한 몇 가지 팁:

- 위의 개발 플로우를 따라 보고, 보내기 전에 Rust는 `cargo fmt`, TypeScript / Vue는 Prettier 기본 설정으로 포매팅해 주세요.
- 새 i18n 키는 `src/i18n/locales/` 아래 4개 로케일 파일 모두에 추가해 주세요. 키를 일치시켜 주시기 바랍니다.
- 새 rathole 설정 필드를 추가하는 경우, Rust 모델(`src-tauri/src/models/`)과 TS 타입(`src/types/rathole.ts`)을 함께 갱신해 주세요.
- 단순 수정 이상의 변경이라면 먼저 이슈를 열어 설계에 대해 같이 이야기해 주세요. 환영합니다.

## 로드맵

검토 중인 방향들 — 도와주시면 감사합니다:

- 다크 모드
- 에디터 내부의 서비스별 상태 칩
- 손쉬운 LAN 구성을 위한 내장 자체 서명 TLS 인증서 생성기
- 완전 오프라인 배포를 위한 선택적 내장 `rathole` 바이너리

## 감사의 말

훌륭한 [rathole](https://github.com/rathole-org/rathole) 프로젝트의 어깨 위에 만들어졌습니다. 그분들의 작업이 없었다면 이 Launcher는 의미가 없었을 겁니다.

## 라이선스

[MIT](LICENSE) — 자유롭게 포크, 배포, 상업적 이용 가능합니다. 릴리스 노트에 한 마디 적어 주시면 감사하지만, 필수는 아닙니다.
