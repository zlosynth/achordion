## Changes

${CHANGES}

## Firmware

Achordion's firmware can be upgraded to obtain new features and bug-fixes. To do so, download the binary from attachments bellow, navigate to [Electro-Smith's web programmer](https://flash.daisy.audio/) and follow the guide to upload the binary throught the "File Upload" tab.

## Input quantization

By default, the input CV snaps to the closest note voltage. This works well with normal inputs or tones that are already quantized to the scale.

If you use a keyboard for CV source, consider trying the alternative firmare `TODO` which maps white keys to all notes of the scale.

Due to white key quantization, the module is quite sensitive to inaccurate CV input of TONE. This is especially notable when the same CV is simultaneously used to control the pitch of another module as well. If you suffer from these issues, try the alternative firmware `achordion-firmware-x.y.z-even-quantization.bin`.
