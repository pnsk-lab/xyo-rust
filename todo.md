# 次の一手: `motion_movesteps` を単独で通す

## Summary
- 現在の構成は `parser` がかなり広く、`compiler` と runtime ABI がまだ薄い段階です。
- 未コミット差分は `MotionMoveSteps` 追加と文字列/runtime 変更が混ざっていますが、これは同時に進めるとスコープが大きすぎます。
- 次にやるべきことは、まずローカルの LLVM 21.1/clang 前提を CI と揃え、そのうえで `motion_movesteps` だけを通る縦切りとして完成させることです。`dtoa`/文字列 ABI はこの次の別タスクに分離します。

## Public APIs / Interfaces
- CLI (`run` / `stats` / `json`) は変更しない。
- 今回の機能追加は `MotionStmt::MoveStep` の IR lowering のみとする。
- 文字列の内部表現と runtime ABI は現状維持に寄せる。`StringStruct` への移行、`xyo_dtoa` の本格接続、新しい string helper ABI は今回のスコープ外にする。

## Implementation Changes
- Toolchain を先に固定する。
  `.github/workflows/build.yml` を正として、repo-local `.llvm/<os-arch>` を使うローカル手順かスクリプトを用意し、`LLVM_SYS_211_PREFIX` と `LLVM_CONFIG_PATH` を必須前提にする。
- 現在の差分は `motion` に集中させる。
  `src/compiler/blocks/motion.rs` の `MoveStep` 実装を仕上げ、`PI / 180.0` へ修正し、既存の回転値から `sin/cos` で `x/y` を更新する方針で固定する。
- 文字列/runtime 側の途中変更は今回から外す。
  `src/compiler/types.rs` / `src/compiler/utils.rs` / `src/compiler/blocks/literal.rs` の pointer-based string 変更と、`bitcodes/c/dtoa.c` の rename は別タスクへ回し、コンパイラが再び build できる基準線を戻す。
- テストとドキュメントを最低限整える。
  `tests/template_cli.rs` をテンプレのまま残さず、`tests/fixtures/simple.sb3` のような最小 fixture で `stats` と `run` の smoke test を作る。
  README と `docs/markdown/blocks.md` の motion IR 対応数を実装結果に合わせる。

## Test Plan
- `llvm-config --version` が `21.1.x` を返す。
- `cargo build` と `cargo test` が通る。
- `motion_movesteps` を含む最小 `.sb3` で `xyo run` が panic せず IR を出す。
- 既存の `SetX` / `ChangeXBy` / `TurnRight` / `PointInDirection` など既実装 motion IR がそのまま通る。
- `stats` の opcode 出力に `motion_movesteps` を含む fixture がテストで確認できる。

## Assumptions
- 直近の目的は「新しい大きな runtime 設計」ではなく「build 可能な縦切りを 1 本増やすこと」。
- 文字列 ABI は今回決め切らず、次タスクであらためて `handle ベース継続` か `StringStruct 移行` を設計する。
- テスト用の小さな `.sb3` fixture をリポジトリに置く方針で進める。

## 次タスク: `StringStruct` 移行手順
1. 先に ABI を固定する。
   Rust/C 境界の文字列型を `StringStruct*` に統一し、`src/compiler/types.rs` の `ptr + len` / `char*` / index 混在をやめる。必要なら `bitcodes/c/lib/` に共有ヘッダを置いて field 順を固定する。
2. IR 側の文字列表現を一本化する。
   `src/compiler/compiler.rs` の `ScratchReturnTypes::String` / `StringLiteral` を `StringStruct` ポインタ基準に寄せ、`src/compiler/blocks/literal.rs` の `Vec<String>` を文字列ハンドル代わりに使う構成を外す。`const_utf16_string` は UTF-16 バッファだけでなく global `StringStruct` も生成してそのポインタを返す。
3. helper 宣言を実装実態に合わせる。
   `src/compiler/types.rs` と `src/compiler/utils.rs` の `num_to_str` / `bool_to_str` 不整合を解消し、数値から文字列への変換は `xyo_dtoa`、bool は `true_literal` / `false_literal` か専用 helper のどちらかに寄せる。
4. C runtime を `StringStruct` 前提に切り替える。
   `bitcodes/c/dtoa.c` は stack buffer を返さない形に直し、`container` / `length` / `hash1` / `hash2` / `references` を埋めた `StringStruct` を返す。あわせて `xyo_str_to_num` / `xyo_str_cmp_*` / `str_to_bool` / `str_is_num` も `StringStruct*` を受ける ABI に揃える。
5. ownership と hash の最小ルールを先に決める。
   初回移行では完全な refcount 実装まで入れなくてもよいが、`literal は不変`、`runtime 生成文字列は解放対象` のような扱いを決めてから進める。hash は生成時に計算して比較 helper で再利用する。
6. coercion と演算子を差し替える。
   `src/compiler/utils.rs` の `scratch_return_to_string` / `scratch_return_to_number` / `scratch_return_to_bool` と `src/compiler/blocks/operator.rs` の文字列比較を `StringStruct*` ベースに揃え、数値比較と文字列比較の分岐だけを残す。
7. 最後に build と挙動を固定する。
   `cargo build` / `cargo test` に加えて、`"10" > "2"`、`"false"` の truthiness、`3.14 -> dtoa -> str_to_num`、非 ASCII 文字列 1 件の round-trip を fixture か unit test で固定する。
