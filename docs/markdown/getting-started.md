# セットアップ

[はじめに](./README.md) / [CLI](./cli.md) / [対応ブロック一覧](./blocks.md) / [アーキテクチャ](./architecture.md)

この章では、`xyo-rust` をローカルでビルドし、手元の `.sb3` ファイルを入力して確認する最短手順を説明します。

## 前提条件

- Rust stable
- LLVM 21.1 系
- `llvm-config` が `PATH` 上にあること

まずは次のコマンドでバージョンを確認しておくと安全です。

```bash
rustc --version
llvm-config --version
```

`llvm-config --version` は `21.1.x` を想定しています。

## ビルド

```bash
cargo build --release
```

ビルド後の実行ファイルは `target/release/xyo` です。

## テスト

```bash
cargo test
```

ヘルプだけ確認したい場合は次でも十分です。

```bash
cargo run -- --help
```

## `.sb3` ファイルを用意する

このリポジトリには現在、配布用の `.sb3` サンプルは含まれていません。Scratch エディタから自分のプロジェクトを書き出して使います。

1. Scratch でプロジェクトを開く
2. 「ファイル」→「コンピューターに保存する」を選ぶ
3. 保存された `.sb3` のパスを控える

以下では、そのファイルを `<path-to-project.sb3>` と表記します。

## 最初に試すコマンド

### 構造だけ見たい場合

```bash
cargo run -- json <path-to-project.sb3>
```

`project.json` をそのまま表示します。

### 規模を知りたい場合

```bash
cargo run -- stats <path-to-project.sb3>
```

次の情報が得られます。

- 入力ファイル名
- 読み込み時間
- ブロック数
- 使用 opcode 一覧

### IR 生成経路を試したい場合

```bash
cargo run -- run <path-to-project.sb3>
```

> **補足**
> `run` はもっとも実験的なコマンドです。入力によっては未実装の opcode や IR 変換で停止することがあります。

## エラーの見方

読み込みや JSON パースに失敗した場合、標準エラー出力には原因チェーンに加えて、可能なら位置情報や周辺コンテキストも表示されます。

## ドキュメントのローカル確認

公開用ドキュメントは Markdown を Taiga 用 XML に変換してからビルドします。

1. `docs/` で Taiga バイナリを用意する
2. `python3 build_taiga_site.py` で XML を生成する
3. `rm -rf build && ./taiga site` で `docs/build/` を生成する
4. `build/index.html` から各ページへ移動する

> **補足**
> GitHub Pages ではワークフローが Taiga バイナリをダウンロードして同じ手順を自動実行します。

[はじめにへ戻る](./README.md)
