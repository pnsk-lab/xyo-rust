# xyo-rust ドキュメント

`xyo-rust` は、Scratch の `.sb3` プロジェクトを Rust で読み込み、構文解析し、LLVM ベースのネイティブ実行へ接続するための実験的プロジェクトです。

## 目的

- Scratch プロジェクトをネイティブ寄りに扱える内部表現へ変換する
- ブロック列からスレッドを抽出する
- LLVM IR を生成し、将来的なネイティブ実行につなげる
- SB3 解析系の検証用ツールチェーンを整える

## 現状

現時点では、以下の流れが実装の中心です。

1. `.sb3` を ZIP として開く
2. `project.json` を取り出す
3. `serde` で Scratch プロジェクト構造へ変換する
4. hat block からスレッドを解析する
5. 一部の式 / 文を LLVM IR へ変換する

完全な Scratch 実行互換はまだ目標段階です。

## 関連リンク

- リポジトリ: <https://github.com/pnsk-lab/xyo-rust>
- README: ルートの `README.md`
