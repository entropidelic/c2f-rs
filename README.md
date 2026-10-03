# c2f-rs

A Rust reimplementation of [Cell2Fire-W](https://github.com/fire2a/C2F-W), a grid-based
forest fire spread simulator.

The goal is a simulator that is simple to read, use and extend. It is rebuilt one layer at
a time, and every layer is checked against outputs of the original C++ simulator.

## Status

Layer 1 of the [roadmap](docs/roadmap.md) is done: the Scott & Burgan surface fire model
and the elliptical fire shape. They reproduce C2F-W's rate of spread towards every
neighbour and the fire intensity of every burnt cell in its `sb-asc` test instance.

There is no simulation yet.

## Documentation

- [Roadmap](docs/roadmap.md): how the simulator works, the plan, and progress towards full
  compatibility with C2F-W.
- [Resources](docs/resources.md): what to read to understand fire spread modelling.

## Code

- `src/scott_burgan.rs`: Scott & Burgan surface fire behaviour (rate of spread, flame
  length, intensity).
- `src/ellipse.rs`: elliptical fire shape and wind direction conventions.
- `tests/sb_asc_reference.rs`: comparison against C2F-W outputs.

## Running the tests

```
cargo test
```
