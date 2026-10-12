# Reference data

Copied from [C2F-W](https://github.com/fire2a/C2F-W) at commit `83b0c3bc`, which is licensed
under the GNU General Public License, version 3.

## sb-asc

A 7×7 Scott & Burgan instance with 100 m cells, no elevation and a constant 10 km/h
southerly wind for 5 hours.

- `input/`: copied from `test/model/sb-asc/`.
- `expected/`: copied from `test/target_results.zip` (`target_results/sb-asc/`), produced by:

  ```
  Cell2Fire --input-instance-folder model/sb-asc --output-folder <out> --nsims 113 \
    --output-messages --grids --out-intensity --sim S --seed 123 --ignitionsLog --scenario 1
  ```

  The `Grids/` folder has not been copied.

Output formats:

- `Messages/MessagesFileNNN.csv`: one line per message, `source cell, target cell, period, rate of spread`.
  Cells are numbered from 1, row by row, starting at the top-left corner. Period *p* means
  the fire arrived after *p*+1 minutes (see "Differences from C2F-W" in the roadmap).
- `SurfaceIntensity/SurfaceIntensityNNN.asc`: cell intensity in kW/m; 0 for unburnt cells.
- `ignition_and_weather_log.csv`: ignition cell and weather file for each simulation.
- `log.txt`: C2F-W's console output, including a results table for each simulation.
