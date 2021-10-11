#!/usr/bin/env python

import subprocess
import sys

import matplotlib.pyplot as plt
import numpy as np


def _render_curve():
    x, y = _build_curve()
    plt.plot(x, y, "-")
    plt.xscale("log")
    plt.yscale("log")
    plt.ylim(-0.1, 1.1)
    plt.show()


def _generate_module():
    MODULE = "lib/src/weighting/table.rs"
    SAMPLES = 1016
    START = 9.0
    MULTIPLE = 236.0

    x, y = _build_curve()

    assert x[0] == START + 1

    with open(MODULE, "w") as f:
        f.write(f"pub const START: f32 = {START};")
        f.write(f"pub const MULTIPLE: f32 = {MULTIPLE};")
        f.write("#[allow(clippy::excessive_precision)]")
        f.write(f"pub const WEIGHTING: [f32; {SAMPLES}] = [")
        for i in range(SAMPLES):
            position = 10 ** (i / MULTIPLE) + START
            sample = np.interp(position, x, y)
            f.write(f"{sample}, ")
        f.write("];")

    _rustfmt(MODULE)


def _rustfmt(path):
    subprocess.run(
        [
            "rustfmt",
            path,
        ],
        stderr=subprocess.DEVNULL,
        check=True,
    )


def _build_curve():
    FREQ = [
        10,
        13,
        16,
        20,
        25,
        32,
        40,
        50,
        63,
        80,
        100,
        125,
        160,
        200,
        250,
        315,
        400,
        500,
        630,
        800,
        1000,
        1250,
        1600,
        2000,
        2500,
        3150,
        4000,
        5000,
        6300,
        8000,
        10000,
        12500,
        16000,
        20000,
    ]

    DB = [
        38.2,
        33.2,
        28.5,
        24.2,
        20.4,
        17.1,
        14.2,
        11.6,
        9.3,
        7.4,
        5.6,
        4.2,
        3.0,
        2.0,
        1.3,
        0.8,
        0.5,
        0.3,
        0.1,
        0.0,
        0.0,
        0.0,
        0.0,
        -0.1,
        -0.2,
        -0.4,
        -0.7,
        -1.2,
        -1.9,
        -2.9,
        -4.3,
        -6.1,
        -8.4,
        -11.1,
    ]
    x = np.array(FREQ)
    y = np.array(DB) - max(DB)
    y = 10 ** (y / 100)  # covert dB to amplitude
    return x, y


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "render":
        _render_curve()
    else:
        _generate_module()
