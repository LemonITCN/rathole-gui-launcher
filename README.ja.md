<div align="center">

# Rathole Launcher

[rathole](https://github.com/rathole-org/rathole) のリバースプロキシ・トンネルを数クリックで管理できるデスクトップアプリ — macOS、Windows、Linux に対応。

[English](README.md) · [简体中文](README.zh-CN.md) · **日本語** · [한국어](README.ko.md)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](../../releases)
[![Tauri](https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri&logoColor=white)](https://tauri.app)
[![Vue 3](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-stable-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org)

[ダウンロード](../../releases) · [主な機能](#主な機能) · [クイックスタート](#クイックスタート) · [ソースからビルド](#ソースからビルド)

</div>

<p align="center">
  <img src="docs/screenshots/launcher-client.png" alt="Rathole Launcher メインウィンドウ" width="780" />
</p>

---

## なぜ作ったか

rathole を構築したことがある方なら、お馴染みの流れだと思います。サーバーに SSH、TOML を 2 つ編集、token をコピー、デーモンを再起動、stderr を眺めて何も間違えていないことを祈る。rathole 自体はとても素晴らしい — ただ、設定の儀式が少しだけ煩わしいだけです。

Rathole Launcher はその儀式を取り除くために存在します。すべての rathole オプションがフォーム項目に対応し、token はサーバー設定から自動で同期され、ログはリアルタイムで流れ、起動・停止・再起動はワンクリック。Launcher 自体がクラッシュしてもトンネルは生き続け、次回起動時にそっと再アタッチされます。

最初に rathole に触れたときに「あったらよかったのに」と思った道具を、自分たちで作りました。よかったらどうぞ。

---

## 主な機能

### 🪟 rathole の全仕様をカバーする視覚的エディタ

ドキュメントに記載されたすべてのオプションをフォームから設定できます。アドレス、token、ハートビート、リトライ、サービスごとの上書き、そしてトランスポート全般(TCP / TLS / Noise / WebSocket)とそれぞれのサブフィールド。詳細設定は標準で折りたたまれているので、シンプルな構成は最後までシンプルなままです。

### 🚀 起動・停止・監視・再起動 — すべて 1 つのウィンドウで

各エディタの最上部に固定されたコントロールバーがあり、起動・停止・保存・再起動のボタンが常に視界の中にあります。ログはその下にリアルタイムで流れ、ストリーム別に色分けされ、インスタンスごとに 1000 行のリングバッファに保存されます。トンネルが動いている間に設定を変更すると、黄色いストリップが現れ、「再起動して新しい設定を適用」がワンクリックで完了します。

### 🤖 自前の rathole を持参 — または取得を任せる

Launcher のそばに rathole バイナリが見当たらないと、`rathole-org/rathole` の最新リリースから OS / CPU に合うアーカイブをダウンロードする提案がアプリ内に表示されます。Apple Silicon にネイティブ ARM ビルドがまだ公開されていないとき、Intel ビルドへ自動でフォールバックし、Rosetta 2 で実行します。新しいリリースが出ていないかも起動のたびに確認し、同じ場所でお知らせします。

### 📋 サーバーの `server.toml` を貼り付けるだけでクライアント設定が完成

サイドバーの **+** をクリック → **サーバー TOML から取り込み** に切り替え → サーバーの設定を貼り付け → **解析** をクリック。サービス名、token、トランスポート設定が自動で抽出されるので、各リモートサービスをローカルアドレスにマッピングするだけです。

### 🛟 自分自身のクラッシュにも、あなたのうっかりにも強い

Launcher が起動した rathole の PID はすべてディスクに永続化されます。Launcher が停電・OOM・誤った `kill -9` で落ちても、rathole の子プロセスはトンネリングを続けます(launchd / init / systemd に再ペアレントされます)。Launcher が戻ってくると、孤立したプロセスを見つけて再アタッチするので、また管理できるようになります。

### 🍴 macOS・Windows・Linux でメニューバーに常駐

ウィンドウを閉じてもアプリは終了せず、macOS のメニューバー、Windows のタスクトレイ、Linux の StatusNotifier に格納されます。そこから任意の設定を開いたり、起動・停止したり、実行中の数を一目で確認できます。本当に終了するときは、すべてのトンネルを優雅にシャットダウンしてからアプリが終了します。

### 🌍 4 言語対応

UI **と** トレイメニューが **简体中文・English・日本語・한국어** に翻訳されています。初回起動時にシステムロケールから自動検出され、起動間で記憶され、左下の言語ピッカーから再起動なしでいつでも切り替えられます。

### 🤝 他のインスタンスを邪魔しない

Launcher は自分が起動したプロセスにしか触れません。マシン上で動いている他の rathole はそのままにし、「設定」ページに「外部」ラベル付きで表示します。ポートが既に使用されているサーバープロファイルを起動しようとすると、Launcher は拒否し、衝突しているアドレスを正確に教えてくれます。

---

## クイックスタート

### 1. インストール

[GitHub Release](../../releases) ごとにビルド済みのインストーラを添付しています。

| プラットフォーム | 取得 | 実行 |
| --- | --- | --- |
| **macOS**(Apple Silicon / Intel) | `*.dmg` | `Applications` にドラッグ、初回は右クリック → **開く** |
| **Windows**(x64) | `*.msi` または `*-setup.exe` | インストーラを実行 |
| **Linux**(x64) | `*.AppImage` / `*.deb` / `*.rpm` | `chmod +x` して実行、または各パッケージマネージャでインストール |

ビルドはデフォルトで未署名なので、初回起動時に OS が警告を出します — 仕様です。気になる場合は自分の証明書で署名すればスキップできます。

### 2. rathole バイナリを用意する

初回起動時に Launcher のそばに `rathole` が見つからないと、次のバナーが表示されます:

> **rathole 実行ファイルが見つかりません**
> GitHub から rathole 0.5.0 を自動ダウンロードできます。または「設定」でパスを指定してください。
> &nbsp; &nbsp; **[ ダウンロード ]** &nbsp; **[ 設定を開く ]**

**ダウンロード** をクリックして数秒待つだけです。自前のバイナリを使いたい場合は、Launcher と同じフォルダに置くか、**設定** で任意のパスを指定してください。

### 3. 設定を作る

2 通りの方法があります。お好きな方をどうぞ:

**手動入力** — サイドバーの **+** をクリックして、フォームを埋めるだけ。各設定は Launcher のそばの `server_conf/` または `client_conf/` にプレーンな TOML として保存されるので、後から手で直したりバージョン管理に入れたりできます。

**サーバー TOML から取り込み** *(クライアント側におすすめ)* — **+** をクリック → **サーバー TOML から取り込み** に切り替え → サーバーファイルを貼り付け → **解析** をクリック。各リモートサービスをローカルアドレス(例:`127.0.0.1:22`)にマップ、**作成** をクリック。完成したクライアント設定が出来上がります。

### 4. 実行

**起動** をクリック。ログが流れ始めます。問題なければ 1〜2 秒以内にサービスがサーバーに登録される様子が見えます。ポートが使用中だったり token が 1 文字違っていたりすれば、すぐに気づけて、どこを直せばいいかも分かります。

---

## ソースからビルド

### 必要なもの

- **Node.js** ≥ 18
- **Rust** stable ≥ 1.77 — [rustup](https://rustup.rs) からインストール
- **Tauri 2 のプラットフォーム依存** — [公式ガイド](https://tauri.app/start/prerequisites/) を参照(macOS は Xcode CLT、Windows は MSVC ビルドツール、Linux は GTK + WebKit2GTK 4.1 + libsoup-3 + libappindicator)

### 開発

```sh
git clone https://github.com/<owner>/rathole-gui-launcher.git
cd rathole-gui-launcher
npm install
npm run tauri:dev
```

初回コンパイルでは約 600 個の Rust crate を取得し、マシンによって 5〜15 分かかります。以降はインクリメンタルで素早く立ち上がります。

### ローカルでバンドル

`scripts/` フォルダに各プラットフォーム向けのワンショット・ビルダーが入っています。各スクリプトはツールチェインの確認、依存関係のインストール、`tauri build` の実行を行い、最終的なバンドルパスを表示します。

| プラットフォーム | コマンド |
| --- | --- |
| **macOS** | `./scripts/build-macos.sh` &nbsp; *(オプション:`--silicon`、`--intel`、`--universal`)* |
| **Linux** | `./scripts/build-linux.sh` |
| **Windows** | `pwsh scripts/build-windows.ps1` |

### GitHub Actions でリリース

`.github/workflows/release.yml` は `v*` タグが push されるたびに **macOS Apple Silicon、macOS Intel、Windows x64、Linux x64** のビルドを並列で行い、ドラフト GitHub Release にすべて添付します:

```sh
# src-tauri/tauri.conf.json のバージョンを上げてコミット、その後:
git tag v0.1.0
git push origin v0.1.0
```

`APPLE_CERTIFICATE`、`TAURI_SIGNING_PRIVATE_KEY` などのリポジトリ secret が設定されていれば、Apple と Windows のコード署名が自動で行われます。設定がなければ未署名のバンドルが生成されます — 初期リリースでは十分使えます。

---

## 技術スタック

| レイヤ | 採用 |
| --- | --- |
| ウィンドウ | [Tauri 2](https://tauri.app) |
| フロントエンド | [Vue 3](https://vuejs.org) `<script setup>`、[Vue Router](https://router.vuejs.org)、[Pinia](https://pinia.vuejs.org)、[Vue I18n](https://vue-i18n.intlify.dev) |
| UI ライブラリ | [Antdv Next](https://github.com/antdv-next/antdv-next) — Ant Design Vue のモダンな Vue 3 リライト |
| バックエンド | Rust + Tokio · [reqwest](https://crates.io/crates/reqwest)(rustls) · [sysinfo](https://crates.io/crates/sysinfo) · [zip](https://crates.io/crates/zip) · [toml](https://crates.io/crates/toml)(順序保持) · [nix](https://crates.io/crates/nix) |
| ビルド | Vite、vue-tsc、tauri-cli |

---

## コントリビュート

Issue と PR、大歓迎です。スムーズに進めるためのちょっとしたヒント:

- 上の開発フローを通したうえで、Rust は `cargo fmt`、TypeScript / Vue は Prettier の既定設定でフォーマットしてください。
- 新しい i18n キーは `src/i18n/locales/` 配下の 4 つのロケールファイルすべてに追加し、キーを揃えてください。
- 新しい rathole 設定フィールドを追加する場合、Rust モデル(`src-tauri/src/models/`)と TS 型(`src/types/rathole.ts`)の両方を更新してください。
- 修正以上の変更については、先に Issue で設計について話しましょう。喜んで議論します。

## ロードマップ

検討中の方向性です。お手伝い歓迎:

- ダークモード
- エディタ内のサービスごとの状態チップ
- 簡易 LAN セットアップ向けの自己署名 TLS 証明書ジェネレータ
- 完全オフライン配布のためのオプションでバンドル可能な `rathole` バイナリ

## 謝辞

素晴らしい [rathole](https://github.com/rathole-org/rathole) プロジェクトの肩の上に立っています。彼らの仕事なくしてはこの Launcher は意味を持ちません。

## ライセンス

[MIT](LICENSE) — フォーク、配布、商用、自由にどうぞ。リリースノートでの一言があれば嬉しいですが、必須ではありません。
