#!/usr/bin/env bash
set -euo pipefail

make manual

cp manual/user/manual_digital.pdf ../zlosynth.github.io/docs/achordion/achordion-user-manual.pdf
cp manual/user/manual_papare.pdf ../zlosynth.github.io/docs/achordion/achordion-user-manual-leaflet.pdf
cp manual/build/manual.pdf ../zlosynth.github.io/docs/achordion/achordion-build-manual.pdf
