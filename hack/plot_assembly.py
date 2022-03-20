#!/usr/bin/env python

from contextlib import contextmanager
import os
import subprocess
import sys
import tempfile

import pcbnew

BOARD_FILE = "hardware/Achordion.kicad_pcb"


def plot_assembly(board):
    layers = [
        ("F_Mask", pcbnew.F_Mask, "50%"),
        ("F_SilkS", pcbnew.F_SilkS, "100%"),
        ("F_Fab", pcbnew.F_Fab, "100%"),
        ("Edge_Cuts", pcbnew.Edge_Cuts, "100%"),
    ]

    with tempfile.TemporaryDirectory() as tmp_dir:
        with plot_controller(board) as pctl:
            popt = pctl.GetPlotOptions()
            popt.SetOutputDirectory(tmp_dir)
            popt.SetPlotReference(True)
            popt.SetPlotValue(True)
            popt.SetPlotInvisibleText(False)
            popt.SetDrillMarksType(pcbnew.PCB_PLOT_PARAMS.SMALL_DRILL_SHAPE)
            popt.SetScale(2)
            popt.SetMirror(False)
            pctl.SetColorMode(True)

            for layer in layers:
                pctl.SetLayer(layer[1])
                pctl.OpenPlotfile(layer[0], pcbnew.PLOT_FORMAT_SVG, "Assembly Layer")
                if pctl.PlotLayer() == False:
                    raise Exception("Failed plotting")

        file_basename = os.path.basename(os.path.splitext(board.GetFileName())[0])

        for layer in layers:
            path = os.path.join(tmp_dir, "{}-{}.svg".format(file_basename, layer[0]))

            with open(path, "r") as f:
                layer_svg = f.read()

            with open(path, "w") as f:
                layer_svg = layer_svg.replace(
                    'width="29.700220cm" height="21.000720cm"',
                    'width="20cm" height="20cm"',
                )
                f.write(layer_svg)

        assembly_html = """
            <html>
                <head>
                    <style>
                        img {
                            position: absolute;
                            left: 0;
                            top: 0;
                            transform: scale(1.7);
                        }
                        .container {
                            position: relative;
                            left: 220px;
                            top: 270px;
                        }
                    </style>
                </head>
        """
        assembly_html += """
                <body>
                    <div class="container">
                        {}
                    </div>
                </body>
            </html>
        """.format(
            "\n".join(
                [
                    '<img src="{0}-{1}.svg" class="black" style="filter: opacity({2});" />'.format(
                        file_basename,
                        layer[0],
                        layer[2],
                    )
                    for layer in layers
                ]
            )
        )

        html_path = os.path.join(tmp_dir, "{}-assembly.html".format(file_basename))

        with open(html_path, "w") as f:
            f.write(assembly_html)

        subprocess.run(
            [
                "chromium-browser",
                "--headless",
                "--print-to-pdf-no-header",
                "--print-to-pdf=hardware/plot/Achordion-Assembly.pdf",
                html_path,
            ],
            stderr=subprocess.DEVNULL,
            check=True,
        )


@contextmanager
def plot_controller(board):
    pctl = pcbnew.PLOT_CONTROLLER(board)
    try:
        yield pctl
    finally:
        pctl.ClosePlot()


if __name__ == "__main__":
    board = pcbnew.LoadBoard(BOARD_FILE)
    plot_assembly(board)
