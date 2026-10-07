# /// script
# requires-python = "==3.14.*"
# dependencies = [
#     "matplotlib>=3.9.4",
#     "numpy>=2.0.2",
# ]
# ///
"""Draws the result of a c2f-rs run: the fuel map next to the fire's arrival time.

Usage:
    uv run scripts/plot_fire.py <results folder> <fuels.asc> [image.png]

The image is saved to <results folder>/fire.png unless another path is given.
uv installs the Python version and packages pinned in plot_fire.py.lock on first use.
"""

import sys

import matplotlib.pyplot as plt
import numpy as np

NO_DATA = -9999
HEADER_LINES = 6


def main():
    if len(sys.argv) not in (3, 4):
        sys.exit(__doc__)
    results_folder, fuels_path = sys.argv[1], sys.argv[2]
    image_path = sys.argv[3] if len(sys.argv) == 4 else f"{results_folder}/fire.png"

    fuels = np.loadtxt(fuels_path, skiprows=HEADER_LINES)
    arrival_minutes = np.loadtxt(f"{results_folder}/arrival_time.asc", skiprows=HEADER_LINES)
    arrival_hours = np.where(arrival_minutes == NO_DATA, np.nan, arrival_minutes / 60)

    figure, (fuel_axis, fire_axis) = plt.subplots(1, 2, figsize=(13, 6))

    fuel_axis.imshow(fuels, cmap="tab20", interpolation="nearest")
    fuel_axis.set_title("Fuel map")

    # Unburnt cells have no value, so the grey background shows through.
    fire_axis.set_facecolor("lightgrey")
    arrival_image = fire_axis.imshow(arrival_hours, cmap="inferno_r", interpolation="nearest")
    figure.colorbar(arrival_image, ax=fire_axis, label="hours after ignition")
    fire_axis.set_title("Arrival time of the fire")

    for axis in (fuel_axis, fire_axis):
        axis.set_xticks([])
        axis.set_yticks([])
    plt.tight_layout()
    plt.savefig(image_path, dpi=110)
    print(f"Image saved to {image_path}")


if __name__ == "__main__":
    main()
