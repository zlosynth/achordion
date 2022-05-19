#!/usr/bin/env bash
set -euo pipefail

make manual

cp manual/user/manual_digital.pdf ../zlosynth.github.io/docs/achordion-user-manual.pdf
cp manual/user/manual_paper.pdf ../zlosynth.github.io/docs/achordion-user-manual-leaflet.pdf
cp manual/build/manual.pdf ../zlosynth.github.io/docs/achordion-build-manual.pdf
