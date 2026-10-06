# c2f-rs

A Rust reimplementation of [Cell2Fire-W](https://github.com/fire2a/C2F-W), a grid-based
forest fire spread simulator.

The goal is a simulator that is simple to read, use and extend. It is rebuilt one layer at
a time, and every layer is checked against outputs of the original C++ simulator.

## Status

Layers 1 to 3 of the [roadmap](docs/roadmap.md) are done. c2f-rs simulates one fire with the
Scott & Burgan fuel model, on flat ground, from a C2F-W instance folder. On C2F-W's `sb-asc`
test instance it reproduces all 113 reference fires: the same burnt cells, messages and
intensities.

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

The results folder gets:

- `messages.csv`: every spread from a cell to a neighbour, with the minute the neighbour
  caught fire and the rate of spread between them.
- `arrival_time.asc`: the minute each cell caught fire. Its burnt cells are the burn scar.
- `intensity.asc` and `flame_length.asc`: the fire of each burnt cell.

The maps are in ESRI ASCII format, with the landscape's map coordinates, so GIS tools
such as QGIS place them on the landscape.

## Documentation

- [Roadmap](docs/roadmap.md): how the simulator works, the plan, and progress towards full
  compatibility with C2F-W.
- [Resources](docs/resources.md): what to read to understand fire spread modelling.

## Code

- `src/main.rs`: the command line program.
- `src/simulation.rs`: fire spread over the grid, one fire period at a time.
- `src/scott_burgan.rs`: Scott & Burgan surface fire behaviour (rate of spread, flame
  length, intensity).
- `src/ellipse.rs`: elliptical fire shape and wind direction conventions.
- `src/instance.rs`: reads an instance folder.
- `src/landscape.rs`: the grid of cells and their fuels, read from `fuels.asc`.
- `src/weather.rs`: weather rows, read from `Weather.csv`.
- `src/output.rs`: writes the results.
- `tests/sb_asc_reference.rs`: runs the program and compares its outputs with C2F-W's.

## Running the tests

```
cargo test
```
