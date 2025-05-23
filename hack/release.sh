#!/usr/bin/env bash
set -euo pipefail

version=${1}

sed -i "s/## Unreleased/## Unreleased\n\n## ${version}/" CHANGELOG.md
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" bank/Cargo.toml
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" eurorack/Cargo.toml
sed -i "s/version =.* # hack\/release.sh$/version = \"${version}\" # hack\/release.sh/" lib/Cargo.toml
sed -i "s/rev .*/rev \"v${version}\")/" hardware/Achordion.kicad_sch
sed -i "s/gr_text \"board .*\" /gr_text \"board v${version}\" /" hardware/Achordion.kicad_pcb
sed -i "s/rev .*/rev \"v${version}\")/" hardware/Achordion.kicad_pcb

make all

rm -rf release
mkdir release

pushd eurorack && cargo +1.63.0 objcopy --release -- -O binary ../release/achordion-firmware-${version}.bin && popd
pushd eurorack && cargo +1.63.0 objcopy --release --features white_key_quantization -- -O binary ../release/achordion-firmware-${version}-white-key-quantization.bin && popd

make manual
cp manual/user/manual_digital.pdf release/achordion-user-manual.pdf
cp manual/build/manual.pdf release/achordion-build-manual.pdf

export CHANGES=$(awk "/## ${version}/{flag=1;next}/## */{flag=0}flag" CHANGELOG.md | awk 'NF')

envsubst < hack/release.tmpl.md > release/notes.md
