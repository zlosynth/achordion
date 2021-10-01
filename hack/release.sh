#!/usr/bin/env bash
set -euo pipefail

version=${1}

sed -i "s/## Unreleased/## Unreleased\n\n## ${version}/" CHANGELOG.md
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" bank/Cargo.toml
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" eurorack/Cargo.toml
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" lib/Cargo.toml
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" puredata/Cargo.toml
sed -i "s/Rev .*/Rev \"v${version}\"/" hardware/Achordion.sch
sed -i "s/gr_text \"board .*\"/gr_text \"board v${version}\"/" hardware/Achordion.kicad_pcb
sed -i "s/rev .*/rev v${version})/" hardware/Achordion.kicad_pcb
