# セットアップ

[はじめに](./README.md) / [CLI](./cli.md) / [対応ブロック一覧](./blocks.md) / [アーキテクチャ](./architecture.md)

この章では、`xyo-rust` をローカルでビルドし、手元の `.sb3` ファイルを入力して確認する最短手順を説明します。

## 前提条件

`xyo-rust` のビルドには以下のツールが必要です。

| ツール | バージョン | 用途 |
| ------ | ---------- | ---- |
| Rust | stable | プロジェクト本体のビルド |
| LLVM | 21.1.x | IR 生成バックエンド |
| `llvm-config` | PATH 上に必要 | ビルドスクリプトが LLVM の設定を取得するため |
| `clang` | PATH 上に必要 | `bitcodes/c/` の C ソースを bitcode へ変換するため |
| `make` / `gmake` | PATH 上にあると便利 | `tools/build_icu_prebuilt.sh` で ICU static archive を構築するときに使う |
| `python3` | PATH 上にあると便利 | ICU ビルド補助スクリプトの整形処理で使う |
| `curl` / `wget` | どちらか 1 つあると便利 | `setup.sh` が ICU source archive を取得するときに使う |

まずは次のコマンドでバージョンを確認しておくと安全です。

```bash
rustc --version
llvm-config --version
clang --version
```

`llvm-config --version` は `21.1.x` を想定しています。

```warn
inkwell は LLVM のバージョンに厳密に依存します。LLVM 21.1.x 以外のバージョンでは Cargo.toml の inkwell の feature を変更する必要があり、単純な差し替えではビルドが通らない場合があります。
```

## LLVM 21.1 のインストール

### Ubuntu / Debian 系

LLVM 公式のインストールスクリプトを使うのが最も確実な方法です。

```bash
# LLVM インストールスクリプトをダウンロードして実行
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 21

# インストール確認
llvm-config-21 --version

# PATH に追加（バージョン番号付きコマンドが入る場合）
export PATH="/usr/lib/llvm-21/bin:$PATH"
llvm-config --version
```

または apt で直接インストールすることもできます。

```bash
sudo apt-get update
sudo apt-get install -y llvm-21 llvm-21-dev clang-21

# シンボリックリンクを作成してデフォルトにする
sudo update-alternatives --install /usr/bin/llvm-config llvm-config /usr/bin/llvm-config-21 100
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-21 100
```

### macOS (Homebrew)

Homebrew で特定バージョンの LLVM をインストールできます。

```bash
# LLVM 21 をインストール
brew install llvm@21

# 環境変数を設定（.zshrc や .bashrc に追記する）
export PATH="$(brew --prefix llvm@21)/bin:$PATH"
export LLVM_CONFIG_PATH="$(brew --prefix llvm@21)/bin/llvm-config"
export CLANG="$(brew --prefix llvm@21)/bin/clang"
export CLANGXX="$(brew --prefix llvm@21)/bin/clang++"

# 確認
llvm-config --version
```

### バージョン確認

インストール後、以下が `21.1.x` を返せば準備完了です。

```bash
llvm-config --version
# 出力例: 21.1.0
```

## ビルド

```bash
git clone https://github.com/pnsk-lab/xyo-rust.git
cd xyo-rust
cargo build --release
```

ビルドには数分かかることがあります。ビルドが成功すると `target/release/xyo` が生成されます。

### ビルド中に行われること

`cargo build` を実行すると、次の処理が順に行われます。

1. **`build.rs` の実行**: `clang` を使って `bitcodes/c/` 配下のトップレベル `.c` ファイルを LLVM bitcode (`bitcodes/bc/`) と LLVM IR (`bitcodes/ll/`) に変換します
2. **Rust コードのコンパイル**: `inkwell` 経由で LLVM ライブラリとリンクしながら Rust コードをコンパイルします
3. **バイナリ生成**: `target/release/xyo` (リリースビルド) または `target/debug/xyo` (デバッグビルド) が生成されます

`cargo build` は `bitcodes/c/` 配下の補助 C コードもあわせて LLVM bitcode に変換します。`str.c` が ICU の Unicode API を使うため、`XYO_ICU_ROOT` または `XYO_ICU_PREBUILT_DIR` を用意してください。

### `configure` が `C compiler cannot create executables` を返す (macOS)

Xcode Command Line Tools か macOS SDK が見えていないと、ICU の `configure` が最初のリンクテストで失敗することがあります。

```bash
xcode-select -p
xcrun --sdk macosx --show-sdk-path
export SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"
```

`setup.sh` と `tools/build_icu_prebuilt.sh`、`tools/build_icu_runtime.sh` は macOS で `SDKROOT` が未設定なら自動検出を試みますが、SDK の場所を手動で固定したい場合は上の `export` を使えます。

### 開発時のビルド

開発中は `--release` なしで高速にビルドできます。

```bash
cargo build
```

デバッグビルドは最適化が弱い代わりに、コンパイルが速くパニック時のスタックトレースが詳細です。

## テスト

```bash
cargo test
```

現時点では `cargo test` がそのままビルド確認と smoke test の入口です。必要になったら `tests/` 配下に fixture を足して広げていくのがよさそうです。

ヘルプだけ確認したい場合は次でも十分です。

```bash
cargo run -- --help
```

## `.sb3` ファイルを用意する

このリポジトリには現在、配布用の `.sb3` サンプルは含まれていません。Scratch エディタから自分のプロジェクトを書き出して使います。

### Scratch エディタからのエクスポート方法

1. [Scratch エディタ](https://scratch.mit.edu/projects/editor/) をブラウザで開く
2. プロジェクトを作成する（または既存のプロジェクトを開く）
3. 上部メニューの「ファイル」→「コンピューターに保存する」を選ぶ
4. 保存された `.sb3` ファイルのパスを控える

### 動作確認に適したプロジェクト

`run` コマンドで最後まで処理を通したい場合は、動き系ブロック（「〇歩動かす」「x座標を〇にする」など）と演算子ブロックだけを使ったシンプルなプロジェクトから始めるとよいです。

現在 `run` で最後まで通る命令の詳細は [対応ブロック一覧](./blocks.md) を参照してください。

## 最初に試すコマンド

### 構造だけ見たい場合

```bash
cargo run -- json <path-to-project.sb3>
```

`project.json` をそのまま標準出力に表示します。Scratch のデータ構造を確認したいときに便利です。

出力は JSON テキストなので、`jq` などと組み合わせると読みやすくなります。

```bash
cargo run -- json my_project.sb3 | jq .
```

### 規模を知りたい場合

```bash
cargo run -- stats <path-to-project.sb3>
```

次の情報が得られます。

| 項目 | 内容 |
| ---- | ---- |
| `File` | 入力した SB3 ファイルのパス |
| `Loading Time` | プロジェクト読み込みにかかった時間 |
| `Block Number` | プロジェクト全体のブロック総数 |
| `Using Op Codes` | 使用されている opcode の一覧 |

出力例:

```
File: my_project.sb3
Loading Time: 2.345ms
Block Number: 42
Using Op Codes: ["motion_movesteps", "motion_turnright", "looks_say", "event_whenflagclicked"]
```

### JIT 実行経路を試したい場合

```bash
cargo run -- run <path-to-project.sb3>
```

> **補足**
> `run` はもっとも実験的なコマンドです。入力によっては未実装の opcode や IR 変換で停止することがあります。
> 移動命令と演算子のみを含むシンプルなプロジェクトから試すことを推奨します。

成功時はターミナルに各スレッドの実行後状態が出力されます。出力例:

```text
JitSpriteState { sprite_x: 100.0, sprite_y: 0.0, sprite_rotate: 0.0 }
```

生成された LLVM IR も確認したい場合は `--emit-llvm` を付けます。IR を保存したあとも JIT 実行は続くため、標準出力には同じように実行後状態が表示されます。

```bash
cargo run -- run <path-to-project.sb3> --emit-llvm out.ll
```

## エラーの見方

読み込みや JSON パースに失敗した場合、標準エラー出力には原因チェーンに加えて、可能なら位置情報や周辺コンテキストも表示されます。

### ファイルが見つからない場合

```
Load error: Failed to open SB3 file: `project.sb3`
  cause: No such file or directory (os error 2)
```

### JSON パースに失敗した場合

```
Load error: Failed to parse `project.json`: `project.sb3`
Path: .targets[0].blocks[blockId_xxx]
Location: line 42, column 15
Context:
  41 | "inputs": {
  42 |   "NUM": [1, "invalid
                  ^
```

- `Path:` は失敗したフィールドへの JSON パスです
- `Location:` は `project.json` 内の行番号・列番号です
- `Context:` はその周辺のテキスト抜粋と、エラー位置を示す `^` 記号です

### 未実装 opcode で停止した場合

`run` コマンドが次のように停止することがあります。

```
Parse error: invalid opcode: looks_sayforsecs: target[0].blocks[blockId_xxx]
```

これは `stats` コマンドで使用 opcode を確認したうえで、[対応ブロック一覧](./blocks.md) と照らし合わせると原因が分かります。

## トラブルシューティング

### `llvm-config: command not found`

LLVM がインストールされていないか、`PATH` に含まれていません。上記の LLVM インストール手順を参照してください。

macOS で Homebrew を使った場合は、`LLVM_CONFIG_PATH` 環境変数を設定する方法も有効です。

```bash
export LLVM_CONFIG_PATH="$(brew --prefix llvm@21)/bin/llvm-config"
cargo build
```

### LLVM バージョンが合わない

```
error: package `inkwell v0.8.0` cannot be built because it requires rustc 1.x.0 or newer...
```

または:

```
error[E0433]: failed to resolve: use of undeclared crate or module `llvm_sys`
```

`Cargo.toml` の `inkwell` の feature でバージョンを確認してください。LLVM 21.1 を使う場合は `llvm21-1` feature が必要です。

### `bitcodes/c/` 配下のコンパイルが失敗する

`clang` が PATH に存在するか確認します。

```bash
which clang
clang --version
```

環境変数 `CLANG` または `LLVM_CONFIG_PATH` を設定してビルドスクリプトにヒントを与えることもできます。

```bash
CLANG=clang-21 cargo build
```

`cargo build` は `bitcodes/c/` 配下の補助 C コードもあわせて LLVM bitcode に変換します。`str.c` は ICU の Unicode API を使うため、`XYO_ICU_ROOT` または `XYO_ICU_PREBUILT_DIR` を用意してください。

既定の ICU source tree が `bitcodes/c/lib/icu/` に無い場合は、`XYO_ICU_ROOT=/path/to/icu` を付けてください。prebuilt archive の配置先を変える場合は `XYO_ICU_PREBUILT_DIR=/path/to/prebuilt` を使えます。

通常は次の 1 コマンドで十分です。

```bash
CLANG=clang-21 CLANGXX=clang++-21 ./setup.sh
```

このコマンドは次をまとめて実行します。

1. vendored ICU を必要に応じて取得する
2. prebuilt ICU static archive を構築する
3. `cargo build --release` を実行する

個別に実行する場合は次を使います。

```bash
./tools/build_icu_prebuilt.sh
cargo build --release
```

## ドキュメントのローカル確認

公開用ドキュメントは Markdown を Taiga 用 XML に変換してからビルドします。

1. `docs/` で Taiga バイナリを用意する
2. `./taiga markdown-dir ./markdown` で Taiga 用 XML を生成する
3. `rm -rf build && ./taiga site` で `docs/build/` を生成する
4. `build/index.html` から各ページへ移動する

> **補足**
> GitHub Pages ではワークフローが Taiga バイナリをダウンロードして同じ手順を自動実行します。

[はじめにへ戻る](./README.md)
