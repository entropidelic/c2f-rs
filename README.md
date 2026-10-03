# c2f-rs

A Rust reimplementation of [Cell2Fire-W](https://github.com/fire2a/C2F-W), a grid-based
forest fire spread simulator.

The goal is a simulator that is simple to read, use and extend. It is rebuilt one layer at
a time, and every layer is checked against outputs of the original C++ simulator.

## Status

Layers 1 and 2 of the [roadmap](docs/roadmap.md) are done. As a library, c2f-rs can read a
fuel raster and a weather file and simulate one fire from a given ignition cell, with the
Scott & Burgan fuel model. On C2F-W's `sb-asc` test instance it reproduces all 113 reference
fires: the same burnt cells and the same messages (cells, time and rate of spread).

There is no command line program or output files yet.

## Documentation

- [Roadmap](docs/roadmap.md): how the simulator works, the plan, and progress towards full
  compatibility with C2F-W.
- [Resources](docs/resources.md): what to read to understand fire spread modelling.

## Code

- `src/simulation.rs`: fire spread over the grid, one fire period at a time.
- `src/scott_burgan.rs`: Scott & Burgan surface fire behaviour (rate of spread, flame
  length, intensity).
- `src/ellipse.rs`: elliptical fire shape and wind direction conventions.
- `src/landscape.rs`: the grid of cells and their fuels, read from `fuels.asc`.
- `src/weather.rs`: weather rows, read from `Weather.csv`.
- `tests/sb_asc_reference.rs`: comparison against C2F-W outputs.

## Running the tests

```
cargo test
```
