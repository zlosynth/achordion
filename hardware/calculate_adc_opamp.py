#!/usr/bin/env python
#
# Read SLOA097: https://www.ti.com/lit/an/sloa097/sloa097.pdf

import sys
import unittest


def main():
    print("""OpAmps scaling for ADC ...

Legend:
  <V_in_min, V_in_max> = range for input voltage (coming from CV)
  <V_out_min, V_out_max> = range for out voltage (coming to ADC)
  R_in = input resistance
  R_f = feedback resistance
  V_ref = reference voltage
  R_ref = resistence in series with the reference voltage
""")

    r_in = 100e3
    v_ref = -10

    v_in_min = -5
    v_in_max = 5
    v_out_min = 0
    v_out_max = 3.3
    r_f, r_ref = calculate(r_in, v_ref, v_in_min, v_in_max, v_out_min, v_out_max)
    print(f"""Scaling from <{v_in_min}, {v_in_max}> to <{v_out_min}, {v_out_max}>:
  R_in = {r_in} Ω
  R_f = {r_f} Ω
  V_ref = {v_ref} V
  R_ref = {r_ref} Ω
""")

    v_in_min = 0
    v_in_max = 10
    v_out_min = 0
    v_out_max = 3.3
    r_f, r_ref = calculate(r_in, v_ref, v_in_min, v_in_max, v_out_min, v_out_max)
    print(f"""Scaling from <{v_in_min}, {v_in_max}> to <{v_out_min}, {v_out_max}>:
  R_in = {r_in} Ω
  R_f = {r_f} Ω
  V_ref = {v_ref} V
  R_ref = {r_ref} Ω
""")


def calculate(r_in, v_ref, v_in_min, v_in_max, v_out_min, v_out_max):
    assert v_in_min < v_in_max, 'Input range must be ascending'
    assert v_out_min < v_out_max, 'Output range must be ascending'

    # This scaler is inverting
    v_out_min, v_out_max = v_out_max, v_out_min

    m = (v_out_max - v_out_min) / (v_in_max - v_in_min)
    b = v_out_min - m * v_in_min

    r_f = round(r_in * abs(m))
    r_ref = round(v_ref * (r_f / abs(b)))

    return r_f, abs(r_ref)


# Test based on reference values from Mutable Instrument schematics
class TestScaling(unittest.TestCase):

    def test_minus_8_to_8(self):
        r_f, r_ref = calculate(
            r_in=100e3,
            v_ref=-10,
            v_in_min=-1.5,
            v_in_max=5.5,
            v_out_min=0,
            v_out_max=3.3,
        )
        self.assertAlmostEqual(r_f, 47e3, delta=47e3*0.05)
        self.assertAlmostEqual(r_ref, 180e3, delta=180e3*0.05)

    def test_minus_5_to_5(self):
        r_f, r_ref = calculate(
            r_in=100e3,
            v_ref=-10,
            v_in_min=-5,
            v_in_max=5,
            v_out_min=0,
            v_out_max=3.3,
        )
        self.assertAlmostEqual(r_f, 33e3, delta=33e3*0.05)
        self.assertAlmostEqual(r_ref, 200e3, delta=200e3*0.05)

    def test_minus_1_5_to_5_5(self):
        r_f, r_ref = calculate(
            r_in=100e3,
            v_ref=-10,
            v_in_min=-1.5,
            v_in_max=5.5,
            v_out_min=0,
            v_out_max=3.3,
        )
        self.assertAlmostEqual(r_f, 47e3, delta=37e3*0.05)
        self.assertAlmostEqual(r_ref, 180e3, delta=180e3*0.05)


if __name__ == '__main__':
    main()
