# scratch-native-runtime

`scratch-native-runtime` is a Rust CLI that loads an `.sb3` project and runs a subset of Scratch by compiling LLVM IR to native code and executing it via a native shared module.

## Design

- `src/project/sb3.rs`: loads `project.json` from the `.sb3` zip.
- `src/engine/ir.rs`: lowers Scratch blocks into a compact Rust IR.
- `src/engine/jit.rs`: emits LLVM IR, can output native object files, and executes via a native shared module backend.
- `src/engine/runtime.rs`: host runtime functions called from generated machine code.
- `src/frontend/gui.rs`: `egui`-based real-time stage preview window (when GUI mode is enabled).
- `src/main.rs`: CLI entry point and GUI/headless orchestration.

The implementation references these `scratch-vm` internals as behavior guides:

- `scratch-vm/src/serialization/sb3.js` (SB3 primitive/input encoding)
- `scratch-vm/src/compiler/irgen.js` (script traversal and input lowering ideas)
- `scratch-vm/src/compiler/enums.js` (opcode naming conventions)

## Supported Blocks (MVP)

- Hat: `event_whenflagclicked`, `event_whencloned`
- Motion: `movesteps`, `pointindirection`, `changexby`, `changeyby`, `setx`, `sety`, `gotoxy`
- Variables: `setvariableto`, `changevariableby`, `data_variable` reporter
- Lists: `item [i] of [list]`, `item # of [x] in [list]`, `length of [list]`, `replace item [i] of [list] with [x]`
- Looks: `looks_say`, `looks_hide`, `looks_show`, `looks_switchcostumeto`, `looks_setsizeto`
- Control: `repeat`, `for each`, `while`, `repeat until`, `wait`, `if`, `if else`
- Control: `forever` (bounded by runtime step budget)
- Control: cloning (`create clone of`, `delete this clone`)
- Operators: `+`, `-`, `*`, `/`, `mod`, `random`, `>`, `<`, `=`, `and`, `or`, `not`, `length`, `letter of`, `mathop`
- Numeric/text primitives (`math_*`, `text`, variable primitive)
- Sensing: `mouse x`, `mouse y`, `mouse down?`, `key [x] pressed?`
- Compatibility no-op statements: `motion_setrotationstyle`, `data_showvariable`, `sensing_setdragmode`, `text2speech_speakAndWait`
- Events: `broadcast`, `broadcast and wait`, `when I receive [message]`
- Procedures: `define`, command/reporter `call`, argument reporters, `return`
- Pen: `pen up`, `clear`, `set pen size`, `set pen color`, `set pen color param`
- Pen: `pen down`, line rasterization and alpha blend to a 480x360 stage pen layer
- Rendering: backdrop + pen layer + sprite composition (including clones and costume/size updates)

Unsupported blocks are currently reported as warnings and skipped.

## Examples

Sample `.sb3` projects are located in the `examples/` directory:

- `examples/3d.sb3` – 3D rendering demo
- `examples/calcPi.sb3` – Pi calculation
- `examples/sort.sb3` – Sorting algorithm visualization
- `examples/bench.sb3` – Performance benchmark
- `examples/linux.sb3` – Linux penguin animation
- `examples/project.sb3`, `examples/project2.sb3` – Test projects

Each example can be run directly:

```bash
cargo run --release -- examples/sort.sb3 --gui --fps 30
```

## Run

```bash
cargo run --release -- path/to/project.sb3
```

Emit a native object file (`.o`) from the lowered Scratch program:

```bash
cargo run --release -- path/to/project.sb3 --emit-native path/to/project.o --emit-only
```

`--emit-native` can be combined with normal execution when you also want to run the project:

```bash
cargo run --release -- path/to/project.sb3 --emit-native path/to/project.o
```

Emit a standalone executable with the `.sb3` payload embedded:

```bash
cargo run --release -- path/to/project.sb3 --emit-executable path/to/project-bin --emit-only
```

The emitted binary can run without passing a `.sb3` argument:

```bash
./path/to/project-bin
```

By default it writes output to `<project-bin>.ppm` (or a custom path as the first positional output argument).

GUI preview is enabled by default and shows composed stage rendering (backdrop/sprites/pen) in real time.
Use `Esc` or window close to stop GUI execution.
When GUI is enabled, execution is paced at `60 FPS` by default and present sync uses `vsync 60 Hz`.
GUI backend uses OpenGL (`eframe`/Glow), and pen layer compositing is GPU-accelerated in the preview path.
Current render FPS is shown in the window title.
GUI window size is fixed at `2x` display size by default.

```bash
cargo run --release -- path/to/project.sb3 --gui
```

Disable GUI (headless):

```bash
cargo run --release -- path/to/project.sb3 --no-gui
```

Set internal stage resolution scale (`1`, `2`, `4`, `8`, `16`) while keeping window size fixed:

```bash
cargo run --release -- path/to/project.sb3 --scale 4
```

Disable/enable present sync:

```bash
cargo run --release -- path/to/project.sb3 --no-vsync
cargo run --release -- path/to/project.sb3 --vsync
```

Set present-sync rate:

```bash
cargo run --release -- path/to/project.sb3 --vsync-fps 144
```

Set FPS (disables turbo mode), for example 30 FPS:

```bash
cargo run --release -- path/to/project.sb3 --fps 30
```

`--fps` controls simulation/update frequency. Rendering/present is synchronized separately (`--vsync*`), so high simulation FPS can run without showing partially drawn frames.

Switch native async scheduler mode (cooperative fibers on native threads):

```bash
cargo run --release -- path/to/project.sb3 --native-async
cargo run --release -- path/to/project.sb3 --no-native-async
```

For `--no-gui`, the default is `--no-native-async` (serial execution).

Use turbo mode explicitly:

```bash
cargo run --release -- path/to/project.sb3 --turbo
```

Enable debug event tracing (event fire + queue + script run):

```bash
SCRATCH_DEBUG=1 cargo run --release -- path/to/project.sb3 --debug
```

`--debug` / `--no-debug` can override `SCRATCH_DEBUG`.  
Trace output includes green flag, broadcast, key press, clone start, queueing, and script execution.

Break on specific broadcast message(s):

```bash
cargo run --release -- path/to/project.sb3 --debug --break-on-message "start"
```

Multiple breakpoints:

```bash
cargo run --release -- path/to/project.sb3 --break-on-message "start" --break-on-message "tick"
```

Environment variable alternative (comma-separated):

```bash
SCRATCH_BREAK_ON_MESSAGE="start,tick" cargo run --release -- path/to/project.sb3
```

The CLI writes the composed stage image as `<project>.ppm` by default (or a custom output path as 2nd arg):

```bash
cargo run --release -- path/to/project.sb3 path/to/output.ppm
```

`control_forever` is guarded by `SCRATCH_STEP_BUDGET` (default `100000`) to keep CLI runs finite:

```bash
SCRATCH_STEP_BUDGET=500000 cargo run --release -- path/to/project.sb3
```

Execution logs include:

- `Execution throughput` (`Operation/s`) based on loop-step budget consumption.
- `Block throughput` (`Block/s`) based on dynamically executed Scratch statement blocks.

SVG costumes/backdrops are rasterized through ImageMagick `convert` when needed. If conversion fails, the runtime prints a warning and skips that asset.
