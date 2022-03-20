#!/usr/bin/env bash
set -euo pipefail

echo 'Open the schematic'
eeschema hardware/Achordion.kicad_sch > /dev/null 2>&1 &

echo 'Wait for the application to start'
until xdotool search --onlyvisible --class 'eeschema' > /dev/null; do sleep 0.01; done

echo 'Focus the window'
sleep 0.1
xdotool windowactivate $(xdotool search --onlyvisible --class 'eeschema')

echo 'Open the plot dialog'
sleep 1
xdotool key alt+f Down Down Down Down Down Down Down Down Down Down Down Return

echo 'Wait for the dialog to pop up'
until xdotool search --onlyvisible --name 'Plot' > /dev/null; do sleep 0.01; done

echo 'Set destination directory'
echo -n 'plot' | xclip -selection clipboard
xdotool key shift+Tab
xdotool key ctrl+v

echo 'Export the PDF, make sure to select the type manually the first time this runs'
xdotool key shift+Tab shift+Tab Return

echo 'Exit'
xdotool key Escape
until xdotool search --onlyvisible --class 'eeschema' > /dev/null; do sleep 0.01; done
sleep 0.1
xdotool key ctrl+q
