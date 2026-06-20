# 開発メモ

[はじめに](./README.md) / [セットアップ](./getting-started.md) / [CLI](./cli.md) / [対応ブロック一覧](./blocks.md) / [アーキテクチャ](./architecture.md)

このページは、`xyo-rust` を変更するときに確認する場所と、生成物・ドキュメント更新の扱いをまとめた開発者向けメモです。

## まず確認するコマンド

ビルド環境が整っているかを確認します。

```bash
rustc --version
llvm-config --version
clang --version
cargo test
```

CLI の読み込み経路だけを手早く確認したい場合は、同梱サンプルを使います。

```bash
cargo run -- stats examples/script.sb3
cargo run -- json examples/script.sb3
```

IR 生成や JIT 実行を確認したい場合は、対象の `.sb3` が [対応ブロック一覧](./blocks.md) の IR 対応範囲に収まっているかを先に確認してください。`examples/script.sb3` は読み込み確認用のサンプルで、IR 未対応の制御ブロックも含みます。

```bash
cargo run -- compile <ir-supported-project.sb3> --output out.ll
cargo run -- run <ir-supported-project.sb3>
```

## 主要な編集箇所

| 変更内容 | 主なファイル |
| -------- | ------------ |
| CLI サブコマンド | `src/cli.rs`, `src/main.rs`, `docs/markdown/cli.md` |
| `.sb3` 読み込み・エラー表示 | `src/sb3.rs`, `docs/markdown/getting-started.md`, `docs/markdown/cli.md` |
| Scratch JSON 型 | `src/types/` |
| opcode のパース | `src/parser/`, `docs/markdown/blocks.md` |
| LLVM IR 生成 | `src/compiler/`, `docs/markdown/blocks.md`, `docs/markdown/architecture.md` |
| JIT 実行 | `src/jit/`, `docs/markdown/architecture.md` |
| C bitcode ヘルパー | `bitcodes/c/`, `build.rs`, `docs/markdown/architecture.md` |

## ブロック対応を増やすとき

新しい opcode の対応を追加する場合は、次の順で確認します。

1. `src/types/mod.rs` の `BlockOpCodes` に opcode が定義されているか確認する
2. `src/parser/blocks/<category>.rs` で `Stmt` または `Expr` へ変換する
3. 必要なら `src/parser/types.rs` に AST バリアントを追加する
4. IR 生成まで対応する場合は `src/compiler/blocks/` と `generate_expr_ir` 側を更新する
5. `docs/markdown/blocks.md` の Parser / IR 対応状況を更新する
6. ユーザーから見える挙動が変わる場合は `README.md`、`docs/markdown/cli.md`、`docs/markdown/architecture.md` も更新する

Parser で通ることと `run` で最後まで通ることは別です。Parser だけ対応した opcode は、`blocks.md` では IR 未対応として明示してください。

## 生成物の扱い

`cargo build` は `build.rs` を通じて `bitcodes/c/` 配下のトップレベル `.c` ファイルから LLVM bitcode と LLVM IR を生成します。

| 生成物 | 扱い |
| ------ | ---- |
| `target/` | Cargo のビルド出力。Git 管理しない |
| `bitcodes/bc/` | `build.rs` が生成する bitcode。Git 管理しない |
| `bitcodes/ll/` | `build.rs` が生成する LLVM IR。Git 管理しない |
| `out.ll` | `compile` の既定出力。必要に応じて手元で確認する |
| `docs/build/` | Taiga が生成する公開サイト。Git 管理しない |
| `docs/site/content/*.xml` | Taiga 用のサイトソース。通常は `docs/markdown/*.md` を編集する |

## ドキュメント更新

通常のドキュメント更新は `docs/markdown/*.md` を編集します。GitHub Pages では `.github/workflows/pages.yml` が Taiga バイナリを取得し、`docs/markdown` からサイトを生成します。

ローカルで確認する場合は `docs/` ディレクトリに Taiga バイナリを置いてから実行します。

```bash
cd docs
./taiga markdown-dir ./markdown
rm -rf build
./taiga site
```

公開サイト用の生成結果は `docs/build/` に出力されます。

## 現在の制約

- Scratch VM と同等のイベントループはまだありません
- `run` は JIT 経路の検証コマンドで、未実装 opcode や未実装式に当たると停止することがあります
- `compile` は LLVM IR を保存しますが、実行可能ファイルの生成までは行いません
- リスト、サウンド再生、ブロードキャスト、クローン、描画の完全な実行は未実装です

前のページ: [アーキテクチャ](./architecture.md)
