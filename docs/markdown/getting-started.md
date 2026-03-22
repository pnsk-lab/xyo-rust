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

1. **`build.rs` の実行**: `clang` を使って `bitcodes/c/dtoa.c` を LLVM bitcode (`bitcodes/bc/`) と LLVM IR (`bitcodes/ll/`) に変換します
2. **Rust コードのコンパイル**: `inkwell` 経由で LLVM ライブラリとリンクしながら Rust コードをコンパイルします
3. **バイナリ生成**: `target/release/xyo` (リリースビルド) または `target/debug/xyo` (デバッグビルド) が生成されます

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

テストは `tests/template_cli.rs` に CLI の動作テストが含まれています。

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

現在 IR 生成まで対応している命令の詳細は [対応ブロック一覧](./blocks.md) を参照してください。

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

### IR 生成経路を試したい場合

```bash
cargo run -- run <path-to-project.sb3>
```

> **補足**
> `run` はもっとも実験的なコマンドです。入力によっては未実装の opcode や IR 変換で停止することがあります。
> 移動命令と演算子のみを含むシンプルなプロジェクトから試すことを推奨します。

成功時はターミナルに LLVM IR テキストが出力されます。出力例:

```llvm
; ModuleID = 'xyo'
source_filename = "xyo"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @thread_0(ptr %0) {
entry:
  ; ... スレッドの IR 命令列
  ret void
}
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

### `bitcodes/c/dtoa.c` のコンパイルが失敗する

`clang` が PATH に存在するか確認します。

```bash
which clang
clang --version
```

環境変数 `CLANG` または `LLVM_CONFIG_PATH` を設定してビルドスクリプトにヒントを与えることもできます。

```bash
CLANG=clang-21 cargo build
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
