# c2f-rs

A Rust reimplementation of [Cell2Fire-W](https://github.com/fire2a/C2F-W), a grid-based
forest fire spread simulator.

We build it one layer at a time, checking each layer against the original C++ simulator.
The aim is to keep the code easy to read and extend.

## Status

Layers 1 to 3 of the [roadmap](docs/roadmap.md) are done. c2f-rs simulates one fire with the
Scott & Burgan fuel model, on flat ground, from a C2F-W instance folder. It matches the
burnt cells, messages and intensities of all 113 reference fires in C2F-W's `sb-asc` test
instance.

## Usage

```
cargo run --release -- <instance folder> --output <results folder> [--ignition <cell>] [--scenario <1-4>]
```

The instance folder has the same layout as in C2F-W: `fuels.asc`, `spain_lookup_table.csv`,
`Weather.csv`, and optionally `Ignitions.csv` and `fuels.prj`. Cells are numbered from 1,
row by row, starting at the top-left corner. Run with `--help` for details.

For example, with the C2F-W repository next to this one:

```
cargo run --release -- ../C2F-W/data/ScottAndBurgan/Vilopriu_2013-asc --output results --scenario 1
```

The program writes four files to the results folder:

- `messages.csv`: spread events between neighbouring cells, with the ignition time and
  rate of spread.
- `arrival_time.asc`: the minute each cell caught fire; also records the burn scar.
- `intensity.asc` and `flame_length.asc`: intensity and flame length at each cell's ignition.

The maps use ESRI ASCII format and the landscape's coordinates, so they line up with
the landscape in GIS tools such as QGIS.

## Watching a fire spread

The viewer is a desktop app that runs the simulation live: press Play to watch the fire
spread, change the wind while it runs, and click on the map to start fires.

```
cargo run --release -p c2f-viewer -- ../C2F-W/data/ScottAndBurgan/Vilopriu_2013-asc
```

Scroll to zoom and drag to pan. The first build takes a minute or two.

## Visualizing a result

`scripts/plot_fire.py` plots the fuel map beside the arrival-time map and saves the
image as `fire.png` in the results folder. Run it with [uv](https://docs.astral.sh/uv/):

```
uv run scripts/plot_fire.py results ../C2F-W/data/ScottAndBurgan/Vilopriu_2013-asc/fuels.asc
```

The script declares its Python version and dependencies; `scripts/plot_fire.py.lock`
pins their versions. On first use, uv installs them in an isolated environment.

## Documentation

- [Roadmap](docs/roadmap.md): simulation model, planned work and C2F-W compatibility.
- [Architecture](docs/architecture.md): library design, viewer and possible integrations.
- [Resources](docs/resources.md): papers, guides and reference implementations.

## Code

- `src/main.rs`: the command line program.
- `src/simulation.rs`: fire spread over the grid, one fire period at a time.
- `src/scott_burgan.rs`: Scott & Burgan surface fire behaviour (rate of spread, flame
  length, intensity).
- `src/ellipse.rs`: elliptical fire shape and wind direction conventions.
- `src/instance.rs`: reads an instance folder.
- `src/landscape.rs`: the grid of cells and their fuels, read from `fuels.asc`.
- `src/weather.rs`: weather rows, read from `Weather.csv`.
- `src/files.rs`: reads text files and reports file paths in errors.
- `src/output.rs`: writes the results.
- `src/error.rs`: the errors the library returns.
- `tests/sb_asc_reference.rs`: runs the program and compares its outputs with C2F-W's.
- `viewer/`: the desktop viewer, a separate package that uses the library.

## Running the tests

```
cargo test
```
