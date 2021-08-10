#!/usr/bin/env python

import sys
import pcbnew

BOARD_NAME = "hardware/Achordion.kicad_pcb"

def plot_gerber(board):
    pctl = pcbnew.PLOT_CONTROLLER(board)
    popt = pctl.GetPlotOptions()

    popt.SetOutputDirectory("plot")
    popt.SetPlotFrameRef(False)
    popt.SetLineWidth(pcbnew.FromMM(0.1))
    popt.SetAutoScale(False)
    popt.SetIncludeGerberNetlistInfo(True)
    popt.SetUseGerberProtelExtensions(False)
    popt.SetScale(1)
    popt.SetMirror(False)
    popt.SetUseGerberAttributes(True)
    popt.SetExcludeEdgeLayer(False)
    popt.SetUseAuxOrigin(True)
    popt.SetSubtractMaskFromSilk(False)
    popt.SetDrillMarksType(pcbnew.PCB_PLOT_PARAMS.NO_DRILL_SHAPE);
    pctl.SetColorMode(True)

    layers = [
        ("F_Cu", pcbnew.F_Cu, "Top layer"),
        ("B_Cu", pcbnew.B_Cu, "Bottom layer"),
        ("B_Paste", pcbnew.B_Paste, "Paste bottom"),
        ("F_Paste", pcbnew.F_Paste, "Paste top"),
        ("F_SilkS", pcbnew.F_SilkS, "Silk top"),
        ("B_SilkS", pcbnew.B_SilkS, "Silk top"),
        ("B_Mask", pcbnew.B_Mask, "Mask bottom"),
        ("F_Mask", pcbnew.F_Mask, "Mask top"),
        ("Edge_Cuts", pcbnew.Edge_Cuts, "Edges"),
    ]

    for layer_info in layers:
        cu_layer = layer_info[1] in (pcbnew.F_Cu, pcbnew.B_Cu)
        popt.SetSkipPlotNPTH_Pads(cu_layer)
        pctl.SetLayer(layer_info[1])
        pctl.OpenPlotfile(layer_info[0], pcbnew.PLOT_FORMAT_GERBER, layer_info[2])
        if pctl.PlotLayer() == False:
            raise Exception("Failed plotting")

    pctl.ClosePlot()

def plot_drill(board):
    drlwriter = pcbnew.EXCELLON_WRITER(board)

    mirror = False
    minimal_header = False
    offset = pcbnew.wxPoint(0, 0)
    merge_npth = False
    drlwriter.SetOptions(mirror, minimal_header, offset, merge_npth)

    metric_fmt = True
    drlwriter.SetFormat(metric_fmt)

    gen_drill = True
    gen_map = False
    drlwriter.CreateDrillandMapFilesSet("hardware/plot", gen_drill, gen_map);


if __name__ == '__main__':
    board = pcbnew.LoadBoard(BOARD_NAME)
    plot_gerber(board)
    plot_drill(board)
