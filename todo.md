# 次の一手: 残りの IR 未対応ブロックを減らす

## Summary
- 現在の構成は `parser` がかなり広く、`compiler` と runtime ABI がまだ薄い段階です。
- 現在の実装は `motion_movesteps` を含む動き系の一部、見た目の say/think と大きさ変更、変数代入と加算、制御の repeat/forever/if/ifelse/wait until、タイマーリセットまで進んでいます。
- 次にやるべきことは、残っている `looks` / `sound` / `event` / `data` / `control` の未対応分岐を減らしつつ、`DataExpr` とリスト系の IR 生成を切り出していくことです。

## Public APIs / Interfaces
- CLI (`run` / `stats` / `json`) は変更しない。
- 文字列の内部表現と runtime ABI は現状維持に寄せる。`StringStruct` への移行、`xyo_dtoa` の本格接続、新しい string helper ABI は別タスクに分離する。

## Implementation Changes
- Toolchain を先に固定する。
  `.github/workflows/build.yml` を正として、repo-local `.llvm/<os-arch>` を使うローカル手順かスクリプトを用意し、`LLVM_SYS_211_PREFIX` と `LLVM_CONFIG_PATH` を必須前提にする。
- 文字列/runtime 側の変更は別タスクで扱う。
  `src/compiler/types.rs` / `src/compiler/utils.rs` / `src/compiler/blocks/literal.rs` の pointer-based string 変更と、`bitcodes/c/dtoa.c` の rename は分離して進める。
- テストとドキュメントを実装結果に合わせて追随させる。
  `tests/template_cli.rs` をテンプレのまま残さず、`tests/fixtures/simple.sb3` のような最小 fixture で `stats` と `run` の smoke test を作る。
  README と `docs/markdown/blocks.md` の IR 対応数を現状に合わせる。

## Test Plan
- `llvm-config --version` が `21.1.x` を返す。
- `cargo build` と `cargo test` が通る。
- `run` の smoke test で、motion / looks / control / sensing / data の現行 IR 対応範囲が崩れていないことを確認する。
- `stats` の opcode 出力に `motion_movesteps` と `control_if` など現行 IR 対応 opcode を含む fixture がテストで確認できる。

## Assumptions
- 直近の目的は「新しい大きな runtime 設計」ではなく「IR 対応範囲を段階的に広げながら build 可能性を保つこと」。
- 文字列 ABI は今は固定しない。runtime 側の置き換えは別タスクで進める。
- テスト用の小さな `.sb3` fixture をリポジトリに置く方針で進める。

## 次タスク: 残りの IR 対応
1. まず未対応の IR 分岐を切り分ける。
   `looks` の残り、`sound`、`event`、`data` のリスト系、`control` の残りをカテゴリ単位で整理する。
2. `DataExpr` の lowering を追加する。
   `DataItemOfList` / `DataItemNumOfList` / `DataLengthOfList` / `DataListContainsItem` を IR に落とし、`DataListContents` の扱いも含めてリストの ABI を決める。
3. 見た目と制御の残りを埋める。
   `LooksStmt` のサイズ以外、`ControlStmt` の `Wait` / `RepeatUntil` / `RepeatWhile` / `AllAtOnce` / `CreateCloneOf` / `DeleteThisClone` / `Stop` / `ForEach` / カウンター系を実装対象にする。
4. 最後に build と smoke test を固定する。
   `cargo build` / `cargo test` に加えて、`motion_movesteps`、`control_if`、`looks_say`、`sensing_resettimer` が混在する fixture で `run` が落ちないことを確認する。
