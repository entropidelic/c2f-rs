# Architecture

c2f-rs keeps the simulation in a library. The command line program and desktop viewer
use it; Python bindings or a web service could do the same.

This document describes the current design and possible changes. The proposed changes
are not scheduled yet.

## Current layout

```
c2f-rs
├── library
│   ├── scott_burgan   fuel + wind → rates of spread, flame length, intensity
│   ├── ellipse        rates of spread in each direction
│   ├── error          library error types
│   ├── files          shared text-file reading and parsing
│   ├── landscape      cell grid and fuels; reads fuels.asc
│   ├── weather        weather rows; reads Weather.csv
│   ├── simulation     step-by-step Simulation and whole-fire simulate()
│   ├── instance       reads a C2F-W instance folder
│   └── output         writes result files
├── main.rs            command line program: read, simulate, write
└── viewer/            separate workspace package: desktop app using egui
```

## What works already

- `main.rs` reads inputs, calls `simulate` and writes results. The viewer drives a
  `Simulation` one step at a time.
- The simulation takes a `Landscape` and weather rows, which can be built in memory,
  and returns a `Fire`. It does no file access or printing.
- Each fire has its own state. Fires can run in parallel, and the same inputs give the
  same result. C2F-W shares global variables between threads.
- Fuel behaviour and ellipse calculations are small, pure functions. They can also be
  used separately, for example to display a fire's shape for a given wind.
- A 508×610 Vilopriu landscape takes about 0.1 seconds to simulate 7 hours of fire:
  roughly 0.25 milliseconds per simulated minute. A 60 fps animation has about
  16 milliseconds per frame.
- The library compiles to WebAssembly without changes.

## Current limits

- Weather can change between steps, and new fires can be started, but cells cannot yet
  be marked as unburnable during a run. Firebreaks and firefighting lines need that API.
- The simulation calls Scott & Burgan directly. Kitral and FBP (roadmap layers 5 and 8)
  will need a common fuel model interface.
- The viewer depends on the whole library, including file I/O and the command line
  dependency (`clap`). A browser app would benefit from separating those dependencies.

## Step-by-step simulation

A caller owns a `Simulation` and advances it when needed:

```rust
let mut simulation = Simulation::new(landscape, &settings)?;
simulation.ignite(cell, &weather)?;
while !simulation.is_over() {
    let changes = simulation.step(&weather);   // one fire period
    draw(&changes.burnt_cells);                // newly ignited cells
    // Change weather or start another fire between steps.
}
let fire = simulation.into_fire();             // all burnt cells and messages
```

- `step` returns newly ignited cells and spread messages, so a UI can redraw just those
  cells.
- Weather is supplied at each step. It can come from a file, UI controls or a forecast.
- `ignite` starts a fire at a cell at any time, allowing several ignition points.
- `Simulation` owns its landscape; `landscape()` provides read access to the grid.
- Cloning a simulation gives an independent copy for comparing scenarios. The propagation
  state consists of a flag per cell and each burning cell's progress towards its neighbours.
- `simulate` runs a whole fire using a table of hourly weather. It wraps `step` in a loop
  and is used by the command line program.

## Errors

The library uses `thiserror` to return error types that callers can match:

- `InputError`: unreadable files, invalid values, missing headers or columns, empty files,
  and raster size mismatches.
- `SimulationError`: unknown fuel models, cells outside the landscape or missing weather.
- `OutputError`: failures when writing results.

Errors include the relevant file, value or cell and have readable messages. The command
line program prints them. It validates CLI-specific input itself, such as cell numbers
starting at 1.

## Desktop viewer

`viewer/` uses egui and the `Simulation` API. It displays the fuel map and colours burnt
cells by time since ignition, making the fire front stand out.

The controls let you:

- Play, pause, step or reset the simulation.
- Set the simulated minutes per frame and the time limit. The default limit is the
  duration covered by the weather file, as in C2F-W.
- Adjust wind speed and direction while the fire runs.
- Change the fuel moisture scenario.
- Click to start fires, scroll to zoom and drag to pan.

The prototype does not yet support firebreaks, side-by-side scenario comparisons,
hourly updates from the weather file, a basemap or running in the browser.

## Proposed changes

### Common fuel model interface

Add this when implementing Kitral (layer 5). Given a cell and weather, each model would
return head and back rates of spread, length-to-breadth ratio, intensity and flame length.
Scott & Burgan, Kitral, Portugal and FBP would implement the same interface.

### Separate core, I/O and CLI packages

The workspace already contains the viewer. If a browser app or another integration
needs fewer dependencies, split the library into:

| Package | Contents | Depends on |
|---|---|---|
| `c2f-core` | Fuel models, ellipse, landscape, weather, simulation | No I/O or CLI dependencies |
| `c2f-io` | Instance reading, result writing; later GeoTIFF | `c2f-core` |
| `c2f-cli` | Command line program | `c2f-core`, `c2f-io`, `clap` |
| UI or Python bindings | Application or language bindings | `c2f-core`, optionally `c2f-io` |

For now, keeping file access and printing outside the simulation is enough.

## Possible integrations

### User interfaces

| Option | Advantages | Limitations |
|---|---|---|
| **egui / eframe** | All Rust; supports desktop and browser; quick to build controls and a grid view | Basic styling; satellite imagery and roads would need extra work |
| **WebAssembly + web page** | Shareable link; JavaScript or TypeScript UI; MapLibre or deck.gl for maps | Two languages; local files must be selected or uploaded |
| **Tauri** | Web UI with Rust simulation and access to local files | Desktop app; more components to maintain |
| **Bevy** | Animation, real-time interaction and 3D terrain | Large dependency; requires adopting its application structure |

### Other uses

| Option | Use |
|---|---|
| **Python bindings** (PyO3 and maturin) | `import c2f` for notebooks, fire2a tools or the Python-based QGIS plugin |
| **Web service** (for example axum) | Server-side simulation with steps streamed to clients; shared results and large batches; requires hosting |
| **Parallel simulations** (rayon) | Thousands of fires with random ignitions and weather for burn probability maps (layer 6) |

### Further viewer features

- Draw firebreaks or firefighting lines.
- Scrub through recorded arrival times.
- Compare scenarios from copies of the same running fire.
- Show fuel, rate of spread, intensity and flame length on hover.

## Suggested order

1. Step-by-step simulation API: done.
2. Common fuel model interface with Kitral (layer 5).
3. First egui viewer: done. Split the library packages when a browser app or another
   integration needs it.
4. Python bindings, if needed for notebooks, fire2a or QGIS.
5. Map-based web interface or web service once the core is stable.
