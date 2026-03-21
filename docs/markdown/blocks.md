# 対応ブロック一覧

[はじめに](./README.md) / [セットアップ](./getting-started.md) / [CLI](./cli.md) / [アーキテクチャ](./architecture.md)

このページは、現時点の `xyo-rust` がどの Scratch opcode を扱えるかを整理した一覧です。

## 凡例

| 列                | 意味                                                   |
| ----------------- | ------------------------------------------------------ |
| Parser            | `project.json` から内部表現へ変換できる段階            |
| IR                | LLVM IR 生成まで現状の実装がある段階                   |
| Hat / Stmt / Expr | それぞれ hat block / 文ブロック / 値ブロックを表します |

```warn
Parser で通ることと、`run` サブコマンドで最後まで通ることは同義ではありません。現状の IR 生成は移動命令と一部演算に限定されています。
```

## 対応状況サマリー

| カテゴリ | Stmt 対応 | Expr 対応 | IR Stmt | IR Expr |
| -------- | --------- | --------- | ------- | ------- |
| 動き | ✅ 全 17 opcode | ✅ 全 8 opcode | 🟡 一部 (8/17) | ❌ なし |
| 見た目 | ✅ 全 21 opcode | ✅ 全 5 opcode | ❌ なし | ❌ なし |
| 音 | ✅ 全 8 opcode | ✅ 全 4 opcode | ❌ なし | ❌ なし |
| イベント | ✅ 全 2 opcode | — | ❌ なし | — |
| 制御 | ✅ 全 15 opcode | ✅ 全 2 opcode | ❌ なし | ❌ なし |
| 調べる | ✅ 全 3 opcode | ✅ 全 22 opcode | ❌ なし | ❌ なし |
| 演算 | — | ✅ 全 18 opcode | — | 🟡 一部 (14/18) |
| データ | ✅ 全 11 opcode | ✅ 全 6 opcode | ❌ なし | ❌ なし |
| 独自ブロック | ✅ 全 1 opcode | ✅ 全 3 opcode | ❌ なし | ❌ なし |
| ペン | ✅ 全 13 opcode | ✅ 全 1 opcode | ❌ なし | ❌ なし |

## Hat block

hat block はスクリプトの起点になるブロックです。各 hat block は `Thread` の `hat` フィールドに格納され、その後に続くブロック列が `stmts` としてパースされます。

| カテゴリ | Scratch での意味                           | opcode                        | Parser | IR       |
| -------- | ------------------------------------------ | ----------------------------- | ------ | -------- |
| イベント | 緑の旗が押されたとき                       | `EventWhenFlagClicked`        | Hat    | 間接対応 |
| イベント | `[キー]` キーが押されたとき                | `EventWhenKeyPressed`         | Hat    | 間接対応 |
| イベント | このスプライトが押されたとき               | `EventWhenThisSpriteClicked`  | Hat    | 間接対応 |
| イベント | ステージが押されたとき                     | `EventWhenStageClicked`       | Hat    | 間接対応 |
| イベント | 背景が切り替わったとき                     | `EventWhenBackdropSwitchesTo` | Hat    | 間接対応 |
| イベント | `[タイマー / 音量]` がしきい値を超えたとき | `EventWhenGreaterThan`        | Hat    | 間接対応 |
| イベント | `[メッセージ]` を受け取ったとき            | `EventWhenBroadcastReceived`  | Hat    | 間接対応 |
| イベント | `[対象]` に触れたとき                      | `EventWhenTouchingObject`     | Hat    | 間接対応 |
| 制御     | クローンされたとき                         | `ControlStartAsClone`         | Hat    | 間接対応 |
| 拡張     | 定義                                       | `ProceduresDefinition`        | Hat    | なし     |

「間接対応」とは、hat block 自体は `Thread` 構造体の一部として生成されるため IR 関数の生成は行われますが、hat block が表すイベント（「旗が押された」「キーが押された」など）を実際にトリガーする仕組みはまだ実装されていないことを意味します。

## 文ブロック

文ブロックは Scratch で何かを「実行する」ブロックです。`Stmt` 列挙型のバリアントとして表現されます。

### 動き

動きカテゴリはスプライトの位置・向きを変更する命令です。IR 生成では、スプライトの X 座標・Y 座標・向きを LLVM グローバル変数として保持し、`store` 命令で値を更新します。

- 対応 opcode:
    - `MotionMoveSteps` — `[数値]` 歩動かす
    - `MotionTurnRight` — `[数値]` 度右に回す
    - `MotionTurnLeft` — `[数値]` 度左に回す
    - `MotionGoTo` — `[対象]` へ行く
    - `MotionGoToXY` — x: `[数値]` y: `[数値]` へ行く
    - `MotionGlideTo` — `[数値]` 秒で `[対象]` へ移動する
    - `MotionGlideSecsToXY` — `[数値]` 秒で x: `[数値]` y: `[数値]` へ移動する
    - `MotionPointInDirection` — `[数値]` 度の方向へ向ける
    - `MotionPointTowards` — `[対象]` へ向ける
    - `MotionChangeXBy` — x座標を `[数値]` ずつ変える
    - `MotionSetX` — x座標を `[数値]` にする
    - `MotionChangeYBy` — y座標を `[数値]` ずつ変える
    - `MotionSetY` — y座標を `[数値]` にする
    - `MotionIfOnEdgeBounce` — もし端に着いたら、跳ね返る
    - `MotionSetRotationStyle` — 回転方法を `[スタイル]` にする
    - `MotionAlignScene` — (NoOp: ステージ整列)
    - `MotionScrollRight` — (NoOp: 右スクロール)
    - `MotionScrollUp` — (NoOp: 上スクロール)
- Parser: `Stmt`
- IR: 一部のみ（位置・向き更新系）
    - `MotionSetX` → `store double %value, ptr @sprite_N_x`
    - `MotionChangeXBy` → `load` + `fadd` + `store` で X 座標を加算
    - `MotionSetY` → `store double %value, ptr @sprite_N_y`
    - `MotionChangeYBy` → `load` + `fadd` + `store` で Y 座標を加算
    - `MotionGoToXY` → X・Y をそれぞれ `store`
    - `MotionTurnRight` → 現在の向きに加算
    - `MotionTurnLeft` → 現在の向きから減算
    - `MotionPointInDirection` → 向きを `store`
- 備考: `AlignScene`, `ScrollRight`, `ScrollUp` は Scratch でも実質 NoOp の命令です

### 見た目

見た目カテゴリはスプライトの外観（コスチューム・大きさ・エフェクト・表示/非表示など）を制御する命令です。現在は IR 生成未対応で、パーサー段階のみです。

- 対応 opcode:
    - `LooksSayForSecs` — `[テキスト]` を `[数値]` 秒言う
    - `LooksSay` — `[テキスト]` と言う
    - `LooksThinkForSecs` — `[テキスト]` を `[数値]` 秒考える
    - `LooksThink` — `[テキスト]` と考える
    - `LooksSwitchCostumeTo` — コスチュームを `[名前]` にする
    - `LooksNextCostume` — 次のコスチュームにする
    - `LooksSwitchBackdropTo` — 背景を `[名前]` にする
    - `LooksNextBackdrop` — 次の背景にする
    - `LooksChangeSizeBy` — 大きさを `[数値]` ずつ変える
    - `LooksSetSizeTo` — 大きさを `[数値]` % にする
    - `LooksChangeEffectBy` — `[効果]` の効果を `[数値]` ずつ変える
    - `LooksSetEffectTo` — `[効果]` の効果を `[数値]` にする
    - `LooksClearGraphicEffects` — 画像効果をなくす
    - `LooksShow` — 表示する
    - `LooksHide` — 隠す
    - `LooksGotoFrontBack` — `[前/後]` に表示する
    - `LooksGoForwardBackwardLayers` — 表示順を `[数値]` 層 `[前/後]` にする
    - `LooksChangeStretchBy` — 横幅を `[数値]` ずつ変える
    - `LooksSetStretchTo` — 横幅を `[数値]` % にする
    - `LooksHideAllSprites` — すべてのスプライトを隠す
    - `LooksSwitchBackdropToAndWait` — 背景を `[名前]` にして待つ
- Parser: `Stmt`
- IR: なし
- 備考: グラフィックエフェクトのパラメータ（`COLOR`, `FISHEYE` など）は `LooksEffects` 列挙型として解釈されます

### 音

音カテゴリはサウンドの再生・音量・エフェクトを制御する命令です。

- 対応 opcode:
    - `SoundPlayUntilDone` — `[サウンド名]` の音を鳴らし終わるまで待つ
    - `SoundPlay` — `[サウンド名]` の音を鳴らす
    - `SoundStopAllSounds` — 音をすべて止める
    - `SoundChangeEffectBy` — `[効果]` の効果を `[数値]` ずつ変える
    - `SoundSetEffectTo` — `[効果]` の効果を `[数値]` にする
    - `SoundClearEffects` — 音の効果をなくす
    - `SoundChangeVolumeBy` — 音量を `[数値]` ずつ変える
    - `SoundSetVolumeTo` — 音量を `[数値]` % にする
- Parser: `Stmt`
- IR: なし
- 備考: 音エフェクト（`PITCH` / `PAN`）は `SoundEffect` 列挙型に変換されます

### イベント

イベントカテゴリはメッセージの送信に関する命令です。

- 対応 opcode:
    - `EventBroadcast` — `[メッセージ]` を送る
    - `EventBroadcastAndWait` — `[メッセージ]` を送って待つ
- Parser: `Stmt`
- IR: なし
- 備考: ブロードキャスト入力（メッセージ名）は `Expr` として解釈されます

### 制御

制御カテゴリは繰り返し・条件分岐・クローン操作・スクリプト停止などの制御構造です。現在はパーサーで `SUBSTACK`（入れ子ブロック列）も再帰的に解析されますが、IR 生成は未対応です。

- 対応 opcode:
    - `ControlWait` — `[数値]` 秒待つ
    - `ControlRepeat` — `[数値]` 回繰り返す
    - `ControlForever` — ずっと（繰り返す）
    - `ControlIf` — もし `[条件]` なら
    - `ControlIfElse` — もし `[条件]` なら…でなければ
    - `ControlWaitUntil` — `[条件]` まで待つ
    - `ControlRepeatUntil` — `[条件]` まで繰り返す
    - `ControlWhile` — `[条件]` の間繰り返す
    - `ControlAllAtOnce` — まとめて実行する
    - `ControlCreateCloneOf` — `[対象]` のクローンを作る
    - `ControlDeleteThisClone` — このクローンを削除する
    - `ControlStop` — `[全部/このスクリプト/他のスクリプト]` を止める
    - `ControlForEach` — `[変数]` を `[数値]` まで繰り返す
    - `ControlIncrCounter` — カウンターを増やす
    - `ControlClearCounter` — カウンターをリセットする
- Parser: `Stmt`
- IR: なし
- 備考: `ControlRepeat`, `ControlForever`, `ControlIf`, `ControlIfElse` は入れ子の `SUBSTACK` を再帰的にパースして `Option<Vec<Stmt>>` として保持します

### 調べる

調べるカテゴリはユーザー入力や外部情報に関する命令です。

- 対応 opcode:
    - `SensingAskAndWait` — `[質問]` と聞いて待つ
    - `SensingSetDragMode` — ドラッグできる / できないにする
    - `SensingResetTimer` — タイマーをリセットする
- Parser: `Stmt`
- IR: なし
- 備考: ドラッグ可否の設定は `SetDraggable { draggable: bool }` として変換されます

### データ

データカテゴリは変数・リストの操作命令です。変数 ID / リスト ID を保持します。

- 対応 opcode:
    - `DataSetVariableTo` — `[変数]` を `[値]` にする
    - `DataChangeVariableBy` — `[変数]` を `[値]` ずつ変える
    - `DataShowVariable` — 変数 `[変数]` を表示する
    - `DataHideVariable` — 変数 `[変数]` を隠す
    - `DataAddToList` — `[値]` を `[リスト]` に追加する
    - `DataDeleteOfList` — `[リスト]` の `[番号]` 番目を削除する
    - `DataDeleteAllOfList` — `[リスト]` のすべてを削除する
    - `DataInsertAtList` — `[値]` を `[リスト]` の `[番号]` 番目に挿入する
    - `DataReplaceItemOfList` — `[リスト]` の `[番号]` 番目を `[値]` に置き換える
    - `DataShowList` — リスト `[リスト]` を表示する
    - `DataHideList` — リスト `[リスト]` を隠す
- Parser: `Stmt`
- IR: なし
- 備考: 変数・リストは名前ではなく ID（文字列）で参照されます。実行時に ID から名前を解決する仕組みは未実装です

### ペン

ペンカテゴリは描画操作命令です。

- 対応 opcode:
    - `PenClear` — 全部消す
    - `PenStamp` — スタンプする
    - `PenDown` — ペンを下ろす
    - `PenUp` — ペンを上げる
    - `PenSetPenColorToColor` — ペンの色を `[色]` にする
    - `PenChangePenColorParamBy` — ペンの `[パラメータ]` を `[数値]` ずつ変える
    - `PenSetPenColorParamTo` — ペンの `[パラメータ]` を `[数値]` にする
    - `PenChangePenSizeBy` — ペンの太さを `[数値]` ずつ変える
    - `PenSetPenSizeTo` — ペンの太さを `[数値]` にする
    - `PenChangePenHueBy` — ペンの色相を `[数値]` ずつ変える
    - `PenSetPenHueToNumber` — ペンの色相を `[数値]` にする
    - `PenChangePenShadeBy` — ペンの明るさを `[数値]` ずつ変える
    - `PenSetPenShadeToNumber` — ペンの明るさを `[数値]` にする
- Parser: `Stmt`
- IR: なし
- 備考: ペンの色・サイズ・色相・明るさの変更が含まれます

### 独自ブロック

独自ブロック（手続き）の呼び出し命令です。

- 対応 opcode: `ProceduresCall`
- Parser: `Stmt`
- IR: なし
- 備考: `mutation` フィールドから引数 ID 一覧と引数値マッピングを復元します。`ProceduresCall { proccode: String, inputs: HashMap<String, Expr> }` として保持されます

## 値ブロック / メニュー / レポーター

値ブロックは何らかの値を返すブロックです。他のブロックの入力として埋め込まれます。`Expr` 列挙型のバリアントとして表現されます。

### 動き

動きカテゴリのレポーター。スプライトの現在位置や向きを取得します。

- 対応 opcode:
    - `MotionXPosition` — x座標
    - `MotionYPosition` — y座標
    - `MotionDirection` — 向き
    - `MotionGoToMenu` — 移動先メニュー
    - `MotionGlideToMenu` — スライド先メニュー
    - `MotionPointTowardsMenu` — 向きメニュー
    - `MotionXScroll` — x スクロール量
    - `MotionYScroll` — y スクロール量
- Parser: `Expr`
- IR: なし
- 備考: メニュー系（`GoToMenu`, `GlideToMenu`, `PointTowardsMenu`）の値は文字列リテラルとして保持されます

### 見た目

スプライト・ステージの外観情報を取得するレポーターです。

- 対応 opcode:
    - `LooksCostumeNumberName` — コスチュームの番号または名前
    - `LooksBackdropNumberName` — 背景の番号または名前
    - `LooksSize` — 大きさ
    - `LooksCostume` — コスチュームメニュー
    - `LooksBackdrops` — 背景メニュー
- Parser: `Expr`
- IR: なし
- 備考: `LooksCostumeNumberName` / `LooksBackdropNumberName` は `number` または `name` の区別を `CostumeStatueTarget` 列挙型として保持します

### 音

音に関する情報を取得するレポーターです。

- 対応 opcode:
    - `SoundVolume` — 音量
    - `SoundSoundsMenu` — サウンドメニュー
    - `SoundBeatsMenu` — 拍数メニュー
    - `SoundEffectsMenu` — 音エフェクトメニュー
- Parser: `Expr`
- IR: なし
- 備考: メニュー系の値は文字列として扱います

### 制御

制御に関するレポーターです。

- 対応 opcode:
    - `ControlCreateCloneOfMenu` — クローン対象メニュー
    - `ControlGetCounter` — カウンターの値
- Parser: `Expr`
- IR: なし
- 備考: `ControlGetCounter` は内部式 `ControlExpr::GetCounter` に変換されます

### 調べる

センサー・環境情報を取得するレポーターです。最も多くの opcode を持つカテゴリです。

- 対応 opcode:
    - `SensingTouchingObject` — `[対象]` に触れた？
    - `SensingTouchingObjectMenu` — 触れた対象メニュー
    - `SensingTouchingColor` — `[色]` に触れた？
    - `SensingColorIsTouchingColor` — `[色]` が `[色]` に触れた？
    - `SensingDistanceTo` — `[対象]` までの距離
    - `SensingDistanceToMenu` — 距離計測対象メニュー
    - `SensingAnswer` — 答え（`ask and wait` の回答）
    - `SensingKeyPressed` — `[キー]` キーが押された？
    - `SensingKeyOptions` — キー選択メニュー
    - `SensingMouseDown` — マウスが押された？
    - `SensingMouseX` — マウスの x 座標
    - `SensingMouseY` — マウスの y 座標
    - `SensingLoudness` — 音量
    - `SensingTimer` — タイマー
    - `SensingOf` — `[スプライト/ステージ]` の `[プロパティ]`
    - `SensingOfObjectMenu` — 対象スプライト選択メニュー
    - `SensingUsername` — ユーザー名
    - `SensingUserid` — ユーザー ID
    - `SensingOnline` — オンライン？
    - `SensingDaysSince2000` — 2000 年 1 月 1 日からの日数
    - `SensingCurrent` — 現在の `[年/月/日/時/分/秒]`
    - `SensingLoud` — 音が大きい？
- Parser: `Expr`
- IR: なし
- 備考: `SensingOf` のプロパティ（`x position`, `y position`, `direction` など）は `StatusTarget` 列挙型、時刻の種類は `TimeTarget` 列挙型として解釈されます

### 演算

数値・文字列・真偽値の演算を行うレポーターです。IR 生成が最も充実しているカテゴリです。

- 対応 opcode:
    - `OperatorAdd` — `[数値]` + `[数値]`
    - `OperatorSubtract` — `[数値]` - `[数値]`
    - `OperatorMultiply` — `[数値]` * `[数値]`
    - `OperatorDivide` — `[数値]` / `[数値]`
    - `OperatorRandom` — `[数値]` から `[数値]` までの乱数
    - `OperatorGt` — `[値]` > `[値]`
    - `OperatorLt` — `[値]` < `[値]`
    - `OperatorEquals` — `[値]` = `[値]`
    - `OperatorAnd` — `[条件]` かつ `[条件]`
    - `OperatorOr` — `[条件]` または `[条件]`
    - `OperatorNot` — `[条件]` ではない
    - `OperatorJoin` — `[文字列]` と `[文字列]` を結合する
    - `OperatorLetterOf` — `[文字列]` の `[番号]` 番目の文字
    - `OperatorLength` — `[文字列]` の文字数
    - `OperatorContains` — `[文字列]` に `[文字列]` が含まれる？
    - `OperatorMod` — `[数値]` を `[数値]` で割った余り
    - `OperatorRound` — `[数値]` を四捨五入
    - `OperatorMathOp` — 数学関数（abs, floor, ceiling, sqrt, sin, cos, tan, asin, acos, atan, ln, log, e^, 10^）
- Parser: `Expr`
- IR: 一部のみ
    - `OperatorAdd` → `fadd double`
    - `OperatorSubtract` → `fsub double`
    - `OperatorMultiply` → `fmul double`
    - `OperatorDivide` → `fdiv double`
    - `OperatorRandom` → `rand` クレートを使った乱数生成 (呼び出し)
    - `OperatorGt` → `fcmp ogt double`
    - `OperatorLt` → `fcmp olt double`
    - `OperatorEquals` → `fcmp oeq double`
    - `OperatorAnd` → `and i1`
    - `OperatorOr` → `or i1`
    - `OperatorNot` → `xor i1 %v, true`
    - `OperatorMod` → `frem double`
    - `OperatorRound` → `llvm.round.f64` 組み込み関数
    - `OperatorMathOp (Abs)` → `llvm.fabs.f64`
    - `OperatorMathOp (Floor)` → `llvm.floor.f64`
    - `OperatorMathOp (Ceil)` → `llvm.ceil.f64`
    - `OperatorMathOp (Sqrt)` → `llvm.sqrt.f64`
    - `OperatorMathOp (Sin/Cos/Tan)` → `llvm.sin.f64` / `llvm.cos.f64` / `tan` 呼び出し
    - `OperatorMathOp (Asin/Acos/Atan)` → `asin` / `acos` / `atan` 呼び出し
    - `OperatorMathOp (LogE)` → `llvm.log.f64`
    - `OperatorMathOp (Log10)` → `llvm.log10.f64`
    - `OperatorMathOp (PowE)` → `llvm.exp.f64`
    - `OperatorMathOp (Pow10)` → `llvm.pow.f64(10.0, x)`
- 備考: 四則演算はすべて `double` (f64) 精度の浮動小数点演算です。整数演算はありません（Scratch の数値型は浮動小数点です）

### データ

変数・リストの参照に関するレポーターです。

- 対応 opcode:
    - `DataItemOfList` — `[リスト]` の `[番号]` 番目
    - `DataItemNumOfList` — `[リスト]` の中の `[値]` の番号
    - `DataLengthOfList` — `[リスト]` の長さ
    - `DataListContainsItem` — `[リスト]` に `[値]` が含まれる？
    - `DataVariable` — 変数の値
    - `DataListContents` — リスト全体（文字列として）
- Parser: `Expr`
- IR: なし
- 備考: 変数参照 (`DataVariable`) とリスト参照 (`DataListContents`) は変数 ID / リスト ID を保持します。実行時の値解決は未実装です

### 独自ブロック

手続きの定義・引数に関するブロックです。

- 対応 opcode:
    - `ProceduresPrototype` — 手続きプロトタイプ（hat block 直下に置かれる）
    - `ArgumentReporterBoolean` — 真偽値引数レポーター
    - `ArgumentReporterStringNumber` — 数値/文字列引数レポーター
- Parser: `Expr`
- IR: なし
- 備考: `ProceduresPrototype` はシグネチャ情報（`proccode`, 引数 ID・名前・デフォルト値一覧、`warp` フラグ）を `ProceduresPrototypeStruct` として保持します

### ペン

ペンのパラメータ選択メニューです。

- 対応 opcode: `PenMenuColorParam`
- Parser: `Expr`
- IR: なし
- 備考: 色パラメータ名（`color`, `saturation`, `brightness`, `transparency`）を文字列リテラルとして扱います

## 入力プリミティブ

ブロック参照以外に、次の入力プリミティブも内部表現へ変換されます。これらは `Expr::Literal(Literal)` として表現されます。

| プリミティブ種別 | Rust での表現 | 例 |
| ---------------- | ------------- | -- |
| 数値 | `Literal::Number(String)` | `"10"`, `"3.14"` |
| テキスト | `Literal::String(String)` | `"Hello"` |
| ブロードキャスト参照 | `Literal::Broadcast { id: String }` | メッセージ ID |
| 変数参照 | `Literal::Variable { target: String }` | 変数 ID |
| リスト参照 | `Literal::List { target: String }` | リスト ID |
| 色 | `Literal::Color { color: String }` | `"#FF0000"` |
| 空入力 | `Literal::Null` | （入力なし） |

数値は文字列として保持されることに注意してください。IR 生成時に `str.parse::<f64>()` で変換されます。

## IR 生成の現状

現在の LLVM IR 生成は、スレッド本体では移動系の一部命令だけを扱います。式側は数値系の一部演算が中心です。

| 層      | 対応内容                                                                                                                                                                                                                                                         |
| ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stmt    | `MotionSetX`, `MotionChangeXBy`, `MotionSetY`, `MotionChangeYBy`, `MotionGoToXY`, `MotionTurnRight`, `MotionTurnLeft`, `MotionPointInDirection`                                                                                                                  |
| Expr    | `OperatorAdd`, `OperatorSubtract`, `OperatorMultiply`, `OperatorDivide`, `OperatorRandom`, `OperatorGreaterThan`, `OperatorLessThan`, `OperatorEq`, `OperatorAnd`, `OperatorOr`, `OperatorNot`, `OperatorMod`, `OperatorRound`, `OperatorCalc`（数学関数） |
| Literal | 数値入力の変換経路あり                                                                                                                                                                                                                                           |

## `run` の前に使える確認コマンド

`run` が成功するかどうかは、使われている opcode に依存します。`stats` コマンドで opcode を確認し、上記の表と照らし合わせてください。

```bash
# 使用 opcode を確認する
cargo run -- stats my_project.sb3

# 確認後、IR 生成を試す
cargo run -- run my_project.sb3
```

前のページ: [CLI](./cli.md)  
次のページ: [アーキテクチャ](./architecture.md)
