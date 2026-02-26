# Block Implementation Audit

- Date: 2026-02-25
- Scope: `src/types/mod.rs` の `BlockOpCodes` と `src/parser/**` 実装を照合し、`scratch-vm/src` との互換性も確認。

## Summary

- `BlockOpCodes` 定義: 154
- parser 側で未対応（未参照含む）: 29
- 上記 29 のうち `scratch-vm` 実装あり: 2
- `scratch-vm` 実装ありだが `BlockOpCodes` enum 未登録: 6

## 1. 未実装（確定）



### 1-2. opcode 未対応一覧（parser で未処理）

- `ControlForEach`
- `ControlGetCounter`
- `ControlIncrCounter`
- `DataListContents`
- `DataVariable`
- `EventWhenTouchingObject`
- `LooksHideAllSprites`
- `LooksSwitchBackdropToAndWait`
- `MotionAlignScene`
- `MotionScrollRight`
- `MotionScrollUp`
- `MotionXScroll`
- `MotionYScroll`
- `PenChangePenHueBy`
- `PenChangePenShadeBy`
- `PenSetPenHueToNumber`
- `PenSetPenShadeToNumber`
- `SensingLoud`

補足:
- 上記は `src/types/mod.rs` の enum には存在し、`src/parser/**` 内の `BlockOpCodes::...` 分岐に未出現。
- `scratch-vm` 側では 28/29 が実装済み（後述の不一致 1 件を除く）。

## 2. 実装ミス（バグ候補）

## 3. 謎実装 / 要確認

### 3-1. `motion_glideto_menu` の存在

- `xyo-rust` には定義・パース実装あり。
  - `src/types/mod.rs:248`
  - `src/parser/blocks/motion.rs:76-82`
- `scratch-vm/src` では opcode 参照が見つからない。

確認観点:
- 互換対象に本当に必要な opcode か。
- 旧仕様/独自拡張由来の可能性。

### 3-2. legacy no-op ブロックの扱い

- `scratch-vm` では no-op 実装のものがある（例: `motion_scroll_right`, `motion_align_scene`）。
  - `scratch-vm/src/blocks/scratch3_motion.js:38-43`
- `xyo-rust` では未実装のため `NotHandledOp` になる。

確認観点:
- 互換性重視なら no-op として受理するか。
- 厳格性重視ならエラーのままにするか。
