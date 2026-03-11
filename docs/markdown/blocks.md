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

## Hat block

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

## 文ブロック

### 動き

- 対応 opcode:
    - `MotionMoveSteps`
    - `MotionTurnRight`
    - `MotionTurnLeft`
    - `MotionGoTo`
    - `MotionGoToXY`
    - `MotionGlideTo`
    - `MotionGlideSecsToXY`
    - `MotionPointInDirection`
    - `MotionPointTowards`
    - `MotionChangeXBy`
    - `MotionSetX`
    - `MotionChangeYBy`
    - `MotionSetY`
    - `MotionIfOnEdgeBounce`
    - `MotionSetRotationStyle`
    - `MotionAlignScene`
    - `MotionScrollRight`
    - `MotionScrollUp`
- Parser: `Stmt`
- IR: 一部のみ
    - `MotionSetX`
    - `MotionChangeXBy`
    - `MotionSetY`
    - `MotionChangeYBy`
    - `MotionGoToXY`
    - `MotionTurnRight`
    - `MotionTurnLeft`
    - `MotionPointInDirection`
- 備考: IR では位置・向き更新系が中心です

### 見た目

- 対応 opcode:
    - `LooksSayForSecs`
    - `LooksSay`
    - `LooksThinkForSecs`
    - `LooksThink`
    - `LooksSwitchCostumeTo`
    - `LooksNextCostume`
    - `LooksSwitchBackdropTo`
    - `LooksNextBackdrop`
    - `LooksChangeSizeBy`
    - `LooksSetSizeTo`
    - `LooksChangeEffectBy`
    - `LooksSetEffectTo`
    - `LooksClearGraphicEffects`
    - `LooksShow`
    - `LooksHide`
    - `LooksGotoFrontBack`
    - `LooksGoForwardBackwardLayers`
    - `LooksChangeStretchBy`
    - `LooksSetStretchTo`
    - `LooksHideAllSprites`
    - `LooksSwitchBackdropToAndWait`
- Parser: `Stmt`
- IR: なし
- 備考: 効果パラメータは列挙値まで解釈します

### 音

- 対応 opcode:
    - `SoundPlayUntilDone`
    - `SoundPlay`
    - `SoundStopAllSounds`
    - `SoundChangeEffectBy`
    - `SoundSetEffectTo`
    - `SoundClearEffects`
    - `SoundChangeVolumeBy`
    - `SoundSetVolumeTo`
- Parser: `Stmt`
- IR: なし
- 備考: `PITCH` / `PAN` を解釈します

### イベント

- 対応 opcode:
    - `EventBroadcast`
    - `EventBroadcastAndWait`
- Parser: `Stmt`
- IR: なし
- 備考: ブロードキャスト入力を式として読み取ります

### 制御

- 対応 opcode:
    - `ControlWait`
    - `ControlRepeat`
    - `ControlForever`
    - `ControlIf`
    - `ControlIfElse`
    - `ControlWaitUntil`
    - `ControlRepeatUntil`
    - `ControlWhile`
    - `ControlAllAtOnce`
    - `ControlCreateCloneOf`
    - `ControlDeleteThisClone`
    - `ControlStop`
    - `ControlForEach`
    - `ControlIncrCounter`
    - `ControlClearCounter`
- Parser: `Stmt`
- IR: なし
- 備考: 入れ子 `SUBSTACK` もたどれます

### 調べる

- 対応 opcode:
    - `SensingAskAndWait`
    - `SensingSetDragMode`
    - `SensingResetTimer`
- Parser: `Stmt`
- IR: なし
- 備考: ドラッグ可否は真偽値へ変換します

### データ

- 対応 opcode:
    - `DataSetVariableTo`
    - `DataChangeVariableBy`
    - `DataShowVariable`
    - `DataHideVariable`
    - `DataAddToList`
    - `DataDeleteOfList`
    - `DataDeleteAllOfList`
    - `DataInsertAtList`
    - `DataReplaceItemOfList`
    - `DataShowList`
    - `DataHideList`
- Parser: `Stmt`
- IR: なし
- 備考: 変数 ID / リスト ID を保持します

### ペン

- 対応 opcode:
    - `PenClear`
    - `PenStamp`
    - `PenDown`
    - `PenUp`
    - `PenSetPenColorToColor`
    - `PenChangePenColorParamBy`
    - `PenSetPenColorParamTo`
    - `PenChangePenSizeBy`
    - `PenSetPenSizeTo`
    - `PenChangePenHueBy`
    - `PenSetPenHueToNumber`
    - `PenChangePenShadeBy`
    - `PenSetPenShadeToNumber`
- Parser: `Stmt`
- IR: なし
- 備考: 色・サイズ・色相・明るさ変更を含みます

### 独自ブロック

- 対応 opcode: `ProceduresCall`
- Parser: `Stmt`
- IR: なし
- 備考: mutation から引数 ID 一覧を復元します

## 値ブロック / メニュー / レポーター

### 動き

- 対応 opcode:
    - `MotionXPosition`
    - `MotionYPosition`
    - `MotionDirection`
    - `MotionGoToMenu`
    - `MotionGlideToMenu`
    - `MotionPointTowardsMenu`
    - `MotionXScroll`
    - `MotionYScroll`
- Parser: `Expr`
- IR: なし
- 備考: メニュー値は文字列リテラルとして保持します

### 見た目

- 対応 opcode:
    - `LooksCostumeNumberName`
    - `LooksBackdropNumberName`
    - `LooksSize`
    - `LooksCostume`
    - `LooksBackdrops`
- Parser: `Expr`
- IR: なし
- 備考: 番号 / 名前の区別まで保持します

### 音

- 対応 opcode:
    - `SoundVolume`
    - `SoundSoundsMenu`
    - `SoundBeatsMenu`
    - `SoundEffectsMenu`
- Parser: `Expr`
- IR: なし
- 備考: メニュー値は文字列として扱います

### 制御

- 対応 opcode:
    - `ControlCreateCloneOfMenu`
    - `ControlGetCounter`
- Parser: `Expr`
- IR: なし
- 備考: カウンタ取得は内部式に変換されます

### 調べる

- 対応 opcode:
    - `SensingTouchingObject`
    - `SensingTouchingObjectMenu`
    - `SensingTouchingColor`
    - `SensingColorIsTouchingColor`
    - `SensingDistanceTo`
    - `SensingDistanceToMenu`
    - `SensingAnswer`
    - `SensingKeyPressed`
    - `SensingKeyOptions`
    - `SensingMouseDown`
    - `SensingMouseX`
    - `SensingMouseY`
    - `SensingLoudness`
    - `SensingTimer`
    - `SensingOf`
    - `SensingOfObjectMenu`
    - `SensingUsername`
    - `SensingUserid`
    - `SensingOnline`
    - `SensingDaysSince2000`
    - `SensingCurrent`
    - `SensingLoud`
- Parser: `Expr`
- IR: なし
- 備考: 対象プロパティや現在時刻種別を列挙値へ変換します

### 演算

- 対応 opcode:
    - `OperatorAdd`
    - `OperatorSubtract`
    - `OperatorMultiply`
    - `OperatorDivide`
    - `OperatorRandom`
    - `OperatorGt`
    - `OperatorLt`
    - `OperatorEquals`
    - `OperatorAnd`
    - `OperatorOr`
    - `OperatorNot`
    - `OperatorJoin`
    - `OperatorLetterOf`
    - `OperatorLength`
    - `OperatorContains`
    - `OperatorMod`
    - `OperatorRound`
    - `OperatorMathOp`
- Parser: `Expr`
- IR: 一部のみ
    - `OperatorAdd`
    - `OperatorSubtract`
    - `OperatorMultiply`
    - `OperatorDivide`
    - `OperatorRandom`
- 備考: `OperatorMathOp` は `abs`, `floor`, `ceiling`, `sqrt`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `ln`, `log`, `e ^`, `10 ^` を解釈します

### データ

- 対応 opcode:
    - `DataItemOfList`
    - `DataItemNumOfList`
    - `DataLengthOfList`
    - `DataListContainsItem`
    - `DataVariable`
    - `DataListContents`
- Parser: `Expr`
- IR: なし
- 備考: 変数 / リスト参照やリスト検索を式化します

### 独自ブロック

- 対応 opcode:
    - `ProceduresPrototype`
    - `ArgumentReporterBoolean`
    - `ArgumentReporterStringNumber`
- Parser: `Expr`
- IR: なし
- 備考: mutation を展開して手続きシグネチャを復元します

### ペン

- 対応 opcode: `PenMenuColorParam`
- Parser: `Expr`
- IR: なし
- 備考: 色パラメータ名を文字列として扱います

## 入力プリミティブ

ブロック参照以外に、次の入力プリミティブも内部表現へ変換されます。

- 数値
- テキスト
- ブロードキャスト参照
- 変数参照
- リスト参照
- 色
- 空入力（`Null`）

## IR 生成の現状

現在の LLVM IR 生成は、スレッド本体では移動系の一部命令だけを扱います。式側は数値系の一部演算が中心です。

| 層      | 対応内容                                                                                                                                        |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Stmt    | `MotionSetX`, `MotionChangeXBy`, `MotionSetY`, `MotionChangeYBy`, `MotionGoToXY`, `MotionTurnRight`, `MotionTurnLeft`, `MotionPointInDirection` |
| Expr    | `OperatorAdd`, `OperatorSubtract`, `OperatorMultiply`, `OperatorDivide`, `OperatorRandom`, `OperatorRandom`, `GreaterThan`                      |
| Literal | 数値入力の変換経路あり                                                                                                                          |

前のページ: [CLI](./cli.md)  
次のページ: [アーキテクチャ](./architecture.md)
