#!/usr/bin/env python

import math
import os
import struct
import wave

TABLE_SIZE = 600
OUTPUT_DIR = 'bank/build/banks/sources'


def _generate_sin_mul_bank():
    for i in range(1, 11):
        _generate_sin_mul(i)


def _generate_sin_mul(size):
    audio = [0.0] * TABLE_SIZE

    for i in range(TABLE_SIZE):
        audio[i] = math.sin(2 * math.pi * (i / TABLE_SIZE))
        for j in range(2, size + 1):
            audio[i] *= math.sin(2 * math.pi * (i / TABLE_SIZE) * j)

    audio = _center_wavetable(audio)
    _save_wavetable(audio, f'sin_mul_{size}.wav')


def _center_wavetable(wavetable):
    mean = sum(wavetable) / len(wavetable)
    return [x - mean for x in wavetable]


def _generate_sin_seq_bank():
    _generate_sin_seq(1)
    _generate_sin_seq(2)
    _generate_sin_seq(3)
    _generate_sin_seq(5)
    _generate_sin_seq(6)
    _generate_sin_seq(7)
    _generate_sin_seq(9)
    _generate_sin_seq(11)
    _generate_sin_seq(13)
    _generate_sin_seq(17)
    _generate_sin_seq(23)
    _generate_sin_seq(31)


def _generate_sin_seq(size):
    audio = [0.0] * TABLE_SIZE

    for i in range(TABLE_SIZE):
        for j in range(1, size + 1):
            audio[i] += math.sin(2 * math.pi * (i / TABLE_SIZE) * j) / size

    _save_wavetable(audio, f'sin_seq_{size}.wav')


def _save_wavetable(wavetable, file_name):
    _normalize_in_place(wavetable)

    with wave.open(os.path.join(OUTPUT_DIR, file_name), 'w') as wav_file:
        number_of_channels = 1
        sample_width = 2
        sample_rate = TABLE_SIZE
        number_of_frames = TABLE_SIZE
        compression_type = 'NONE'
        compression_name = 'not compressed'
        wav_file.setparams((
            number_of_channels,
            sample_width,
            sample_rate,
            number_of_frames,
            compression_type,
            compression_name,
        ))

        for sample in wavetable:
            wav_file.writeframes(struct.pack('h', int(sample * 32767)))


def _normalize_in_place(data):
    max_delta = 0
    for sample in data:
        if abs(sample) > max_delta:
            max_delta = abs(sample)

    for i, sample in enumerate(data):
        data[i] = sample / max_delta * 0.9


def main():
    _generate_sin_mul_bank()
    _generate_sin_seq_bank()


if __name__ == '__main__':
    main()
