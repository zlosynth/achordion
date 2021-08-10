#!/usr/bin/env bash
set -euo pipefail

echo 'Open the schematic'
eeschema hardware/Achordion.sch &

echo 'Wait for the application to start'
until xdotool search --onlyvisible --name 'eeschema'; do sleep 0.01; done

echo 'Focus the window'
xdotool search --onlyvisible --name 'eeschema' windowfocus

echo 'Open the plot dialog'
xdotool key alt+f l

echo 'Wait for the dialog to pop up'
until xdotool search --onlyvisible --name 'Plot'; do sleep 0.01; done

echo 'Set destination directory'
echo -n 'plot' | xclip -selection clipboard
xdotool key shift+Tab
xdotool key ctrl+v

echo 'Export the PDF, make sure to select the type manually the first time this runs'
xdotool key shift+Tab shift+Tab shift+Tab Return

echo 'Exit'
xdotool key Escape
until xdotool search --onlyvisible --name 'eeschema'; do sleep 0.01; done
sleep 0.1
xdotool key ctrl+q
