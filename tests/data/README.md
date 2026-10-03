# Reference data

Copied from the C2F-W repository (https://github.com/fire2a/C2F-W, commit `83b0c3bc`).

## sb-asc

A 7x7 Scott & Burgan instance with 100 m cells and no elevation. The weather is 10 km/h
wind from the south for 5 hours.

- `input/`: copied from `test/model/sb-asc/`.
- `expected/`: copied from `test/target_results.zip` (`target_results/sb-asc/`), produced by:

  ```
  Cell2Fire --input-instance-folder model/sb-asc --output-folder <out> --nsims 113 \
    --output-messages --grids --out-intensity --sim S --seed 123 --ignitionsLog --scenario 1
  ```

  The `Grids/` folder is not copied yet.

Output formats:

- `Messages/MessagesFileNNN.csv`: one line per message, `source cell, target cell, period, rate of spread`.
  Cells are numbered from 1, row by row, starting at the top-left corner.
- `SurfaceIntensity/SurfaceIntensityNNN.asc`: intensity of every cell in kW/m, 0 if it did not burn.
- `ignition_and_weather_log.csv`: ignition cell and weather file of every simulation.
