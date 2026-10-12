# c2f-rs

A Rust reimplementation of [Cell2Fire-W](https://github.com/fire2a/C2F-W), a grid-based
forest fire spread simulator.

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

## Running the tests

```
cargo test
```

## Credits

c2f-rs is based on these projects and papers:

- [Cell2Fire](https://github.com/cell2fire/Cell2Fire), by Cristóbal Pais, Jaime Carrasco,
  David Martell, Andrés Weintraub and David L. Woodruff. The model is described in
  [Pais et al. (2021)](https://doi.org/10.3389/ffgc.2021.692706).
- [C2F-W](https://github.com/fire2a/C2F-W), by the fire2a team, which adds the Scott & Burgan
  and Kitral fuel models to Cell2Fire. The Scott & Burgan version is described in
  [González-Olabarria et al. (2023)](https://doi.org/10.3389/ffgc.2023.1071484).

Two parts of this repository are copied from C2F-W:

- the Scott & Burgan coefficient tables in `src/scott_burgan/coefficients.rs`, taken from
  C2F-W's `Cell2Fire/FuelModelSpain.cpp`;
- the reference data in `tests/data/`, used to check that c2f-rs gives the same results
  (see [tests/data/README.md](tests/data/README.md)).

The rest of the code is new, but it follows how C2F-W works, as described in the
[roadmap](docs/roadmap.md). The [resources](docs/resources.md) list the other papers the
model relies on, such as Scott & Burgan (2005) for the fuel models and Anderson (1983) for
the fire shape.

## Licence

c2f-rs is licensed under the GNU General Public License, version 3, the same licence as
Cell2Fire and C2F-W. See [LICENSE](LICENSE) for the full text.

In short, you can use, study, change and share c2f-rs. If you share it, or a changed version
of it, you have to share the source code too, under the same licence.
