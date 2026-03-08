# アーキテクチャ

## 全体フロー

`xyo-rust` の大まかな処理フローは次のとおりです。

1. `src/sb3.rs` で `.sb3` を読み込む
2. `ScratchProject` へデシリアライズする
3. `src/parser/` で hat block からスレッドを組み立てる
4. `src/compiler/` で LLVM IR を生成する
5. 将来的にはネイティブ実行へ接続する

## モジュール概要

### `src/sb3.rs`

- ZIP アーカイブとして `.sb3` を開く
- `project.json` を読み込む
- UTF-8 / JSON / パス付きエラーを整形する

### `src/parser/`

- Scratch ブロックを `Stmt` / `Expr` に変換する
- hat block を検出し、スレッド単位に解析する
- opcode 種別ごとに処理を分割する

### `src/compiler/`

- `inkwell` を使って LLVM コンテキストとモジュールを構築する
- スレッドごとに関数を生成する
- 一部の式や命令を LLVM IR に変換する
- ターゲット CPU 情報に合わせて最適化パスを適用する

## 制約

現時点では、全 opcode の完全実装や Scratch VM と同等の実行モデルは未完です。そのため、`run` サブコマンドは「コンパイル経路の検証ツール」として考えるのが適切です。
