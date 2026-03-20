# xyo-rust

`xyo-rust` は、Scratch の `.sb3` プロジェクトを Rust で読み込み、構文解析し、LLVM IR 生成へ接続する実験的なランタイム / コンパイラ基盤です。

いまの主眼は **SB3 ローダー・パーサー・IR 生成経路の検証** にあります。Scratch VM と同等の完全実行を目指す段階ではなく、まずは「Scratch プロジェクトをどこまで静的に扱えるか」を試すための土台が実装されています。

## 現在できること

- `.sb3` から `project.json` を取り出す
- `project.json` をそのまま標準出力へ表示する
- ブロック数や使用 opcode 一覧などの統計を確認する
- hat block を起点にスレッドを抽出し、内部表現へ変換する
- 一部の式・命令を LLVM IR へ落とし込む
- JSON パース失敗時に、発生位置と周辺コンテキストを表示する

## まだ開発途中のこと

- Scratch opcode の網羅的な実装
- 実行ランタイムの完成
- 生成した IR から実行可能ファイルへつなぐフロー
- 互換性検証とリグレッションテストの拡充
- `run` サブコマンドの未実装命令に対する安全なフォールバック

## 必要環境

- Rust stable
- LLVM 21.1 系
- `llvm-config` が利用可能な環境
- `clang` が利用可能な環境

`inkwell` を使っているため、LLVM のメジャー・マイナー差異には注意が必要です。作業前に `llvm-config --version` が `21.1.x` を返すことを確認してください。

## クイックスタート

### ビルド

```bash
cargo build --release
```

`cargo build` は `build.rs` を通じて `bitcodes/` 配下の C コードも再生成します。

### テスト

```bash
cargo test
```

### CLI ヘルプ

```bash
cargo run -- --help
```

## CLI

生成される実行ファイル名は `xyo` です。開発中は `cargo run -- ...` で、そのまま試せます。

### 統計情報を見る

```bash
cargo run -- stats <path-to-project.sb3>
```

出力内容:

- 入力ファイル名
- 読み込み時間
- ブロック数
- 使用されている opcode 一覧

### `project.json` を表示する

```bash
cargo run -- json <path-to-project.sb3>
```

### 解析と IR 生成を試す

```bash
cargo run -- run <path-to-project.sb3>
```

`run` は現状もっとも実験的なコマンドです。LLVM IR 生成の途中に未実装分岐が残っているため、複雑なプロジェクトでは失敗することがあります。まずは `stats` と `json` で入力を確認してから使うのが安全です。

## 入力ファイルについて

このリポジトリには現在、配布用の `.sb3` サンプルは含まれていません。Scratch エディタでプロジェクトを作成し、**「コンピューターに保存する」** で `.sb3` を書き出して入力に使ってください。

## プロジェクト構成

- `src/main.rs`: CLI エントリポイントとエラー出力
- `src/cli.rs`: サブコマンド定義
- `src/sb3.rs`: `.sb3` / `project.json` の読み込みと詳細エラー整形
- `src/parser/`: Scratch ブロック列を `Stmt` / `Expr` に変換
- `src/compiler/`: LLVM IR 生成
- `tests/`: CLI テスト
- `docs/`: Markdown ソースと Taiga サイト生成ファイル

## CI

- `.github/workflows/build.yml`: GitHub Release 公開時の Rust バイナリのマルチプラットフォームビルド
- `.github/workflows/bitcodes.yml`: `bitcodes/` 配下の C ソースから `.bc` / `.ll` を自動生成

## Bitcodes

`bitcodes/` には LLVM bitcode と LLVM IR の生成元 C コードがあります。`cargo build` 時に `bitcodes/bc/` と `bitcodes/ll/` が更新され、Git には含めません。

ローカルで再生成するには次を使います。

```bash
./bitcodes/build.sh
```

古い出力を消して作り直す場合は `--clean`、全部強制再生成する場合は `--force` を付けます。

## ライセンス

MIT
