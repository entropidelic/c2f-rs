# Architecture

c2f-rs should be the core of the simulation: a library that knows how fire spreads, and
nothing else. Other programs consume it: today the command line program, and later perhaps a
user interface that shows the fire spreading in real time and lets us interact with it,
Python bindings, or a web service.

This document looks at how well the code fits that goal today, what would need to change,
and the options for consumers. Nothing here is decided yet.

## How the code is organised today

```
c2f-rs (one package)
├── library
│   ├── scott_burgan   fuel model: fuel + wind → rates of spread, flame length, intensity
│   ├── ellipse        fire shape: rates of spread in every direction
│   ├── landscape      the grid of cells and fuels   (+ reads fuels.asc)
│   ├── weather        weather rows                  (+ reads Weather.csv)
│   ├── simulation     a running fire that advances step by step, and simulate() for a whole fire
│   ├── instance       reads a C2F-W instance folder
│   └── output         writes the result files
└── program (main.rs)  command line: reads an instance, simulates, writes the results
```

## What already fits the goal

- **The simulation is a library, and the program is only a consumer.** `main.rs` reads
  files, calls `simulate`, and writes files. A user interface would drive a `Simulation`
  step by step instead, without the files.
- **A fire advances step by step** (see below), so a program can draw it, pause it, or
  change the weather between steps.
- **The simulation does not read or write files, nor print anything.** It takes a
  `Landscape` and weather rows, which can be built in memory, and returns a `Fire`. File
  reading and writing live in separate functions and modules.
- **No global state.** Every fire is independent, so many fires can run in parallel, and
  the same inputs always give the same fire. C2F-W shares global variables between its
  threads.
- **The physics is made of small pure functions** (fuel model, ellipse), easy to test and to
  call from anywhere, for example to show a fire's shape for a given wind.
- **It is fast enough for real time.** The 508×610 Vilopriu landscape burns for 7 hours of
  simulated time in 0.1 seconds: about a quarter of a millisecond per simulated minute. An
  animation at 60 frames per second has 16 milliseconds per frame.
- **The library already compiles to WebAssembly**, the format browsers run, without changes.

## What does not fit yet

1. **Few ways to change a running fire yet.** The weather can change at every step, and
   new fires can be started, but there is no way to mark cells as unburnable (firebreaks,
   firefighting lines) during a fire. It is easy to add when a user interface needs it.
2. **The fuel model is hard-wired.** The simulation calls Scott & Burgan directly. Adding
   Kitral or FBP (layers 5 and 8) needs a common way to ask "how does this cell burn under
   this weather?".
3. **Everything is one package.** File reading and writing, and the command line
   dependency (`clap`), are compiled into every consumer, including a browser app that has
   no files and no command line.
4. **Errors are plain text.** A program can show them, but cannot easily tell a missing file
   from an invalid fuel code. That matters once other programs depend on the library.

## A fire that advances step by step (done)

A running fire is a value that consumers hold and drive:

```rust
let mut simulation = Simulation::new(&landscape, &settings)?;
simulation.ignite(cell, &weather)?;
while !simulation.is_over() {
    let changes = simulation.step(&weather);   // one fire period, under this weather
    draw(changes.burnt_cells);                 // only the cells that just caught fire
    // between steps: change the weather, start another fire, ...
}
let fire = simulation.into_fire();             // every burnt cell and message
```

- `step` advances one fire period and returns what changed: the cells that caught fire and
  the messages. A user interface only redraws those cells.
- The weather is given at every step, so it can come from a file, a slider in a user
  interface, or a weather forecast.
- `ignite` starts a fire at any cell, at any time, so a fire can have several origins.
- A `Simulation` can be copied, to let two copies of the same fire continue differently and
  compare them ("what if" scenarios). Its state is small: a flag per cell, and the burning
  cells with how far their fire has advanced towards each neighbour.
- `simulate` runs a whole fire with a table of hourly weather, as a short loop over `step`.
  The command line program uses it.

## Proposed changes

### A common fuel model interface

When Kitral is added (layer 5), introduce a common interface for fuel models:
given a cell and the weather, return the head and back rates of spread, the length-to-breadth
ratio, the intensity and the flame length. Scott & Burgan, Kitral, Portugal and FBP each
implement it. The simulation only knows the interface.

### Separate packages, when a second consumer appears

Split the package into a Cargo workspace:

| Package | Contents | Depends on |
|---|---|---|
| `c2f-core` | fuel models, ellipse, landscape, weather, simulation | nothing |
| `c2f-io` | reading instance folders, writing results; later GeoTIFF | `c2f-core` |
| `c2f-cli` | the command line program | `c2f-core`, `c2f-io`, `clap` |
| a user interface, Python bindings, ... | | `c2f-core`, maybe `c2f-io` |

This is cheap to do, but only worth it when there is a second consumer. Until then, keeping
the simulation free of file access and printing (as it is) is enough.

## Options for consumers

### User interfaces

| Option | What it is | Pros | Cons |
|---|---|---|---|
| **egui / eframe** | A simple Rust user interface library | All Rust. One code base runs as a desktop app and in the browser. Quick to build sliders, buttons and a grid that redraws every step | Basic looks. No real map underneath (no satellite imagery or roads) unless we add it |
| **WebAssembly + web page** | The core compiled for the browser, with the interface in JavaScript or TypeScript, for example a canvas, or MapLibre for a real map | Nothing to install, easy to share a link. Real maps with MapLibre or deck.gl | Two languages. Files must be uploaded or downloaded, the browser cannot read local folders freely |
| **Tauri** | A desktop app with a web interface and Rust behind it | Web interface with full access to local files, and native speed | Desktop only, more moving parts |
| **Bevy** | A Rust game engine | Excellent for real-time interaction, smooth animation, 3D terrain | Large, and it organises programs its own way. Probably more than we need |

### Other consumers

| Option | What it is | Why it could matter |
|---|---|---|
| **Python bindings** (PyO3 and maturin) | `import c2f` in Python | Fits how the field works: notebooks, the fire2a tools, and the QGIS plugin, which is written in Python. Could make c2f-rs a drop-in engine for existing users |
| **Web service** (for example axum) | Simulations run on a server and stream their steps to web clients | Many users, heavy batches of simulations, shared results. Needs hosting |
| **Many simulations in parallel** (rayon) | Thousands of fires with random ignitions and weather | Burn probability maps (layer 6). The lack of global state makes this straightforward |

### Interactions a user interface could offer

- Play, pause, speed up, and step through the fire minute by minute.
- Click to start a fire, or several.
- Change the wind speed and direction while the fire runs.
- Draw firebreaks or firefighting lines, and see the fire react.
- Scrub back in time. This is cheap, because the arrival time of every cell is already
  recorded.
- Compare two "what if" scenarios side by side, from a copy of the same running fire.
- Show a cell's fire behaviour on hover: fuel, rate of spread, intensity, flame length.

## Suggested order

1. ~~**The step-by-step fire.**~~ Done.
2. **The common fuel model interface**, together with Kitral (layer 5).
3. **A first user interface prototype with egui**, as a separate package in a workspace,
   which is when the package split happens. It is the shortest path to seeing and playing
   with the fire, and it can run in the browser too.
4. **Python bindings**, if c2f-rs is to be used from notebooks, fire2a or QGIS.
5. A map-based web interface, or a web service, once the core is stable.
