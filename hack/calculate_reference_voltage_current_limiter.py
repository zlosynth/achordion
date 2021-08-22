#!/usr/bin/env python
#
# Read SLYC147A: https://www.ti.com/lit/eb/slyc147a/slyc147a.pdf

import sys
import unittest


def main():
    print(
        """Refernce voltage resistor for current limiting

Legend:
  R_lim = limiting resistor value
"""
    )

    r_lim_min, r_lim_max = calculate(
        v_in=-12,
        v_ref=-10,
        i_q_min=45e-6, # LM4040
        i_q_max=15e-3, # LM4040
        r_refs=[100e3] * 2 + [200e3] * 4
    )

    r_lim_min = round(r_lim_min)
    r_lim_max = round(r_lim_max)

    print(f"R_lim must between {r_lim_min} and {r_lim_max} Ω")

def calculate(v_in, v_ref, i_q_min, i_q_max, r_refs, *args):
    v_in = abs(v_in)
    v_ref = abs(v_ref)

    i_load = 0
    for r_ref in r_refs:
        i_load += v_ref / r_ref

    r_lim_min = (v_in - v_ref) / (i_load + i_q_max)
    r_lim_max = (v_in - v_ref) / (i_load + i_q_min)

    return r_lim_min, r_lim_max


# Test based on reference values from Mutable Instrument schematics
class TestCalculation(unittest.TestCase):
    def test_plaits(self):
        r_lim_min, r_lim_max = calculate(
            v_in=-12,
            v_ref=-10,
            i_q_min=45e-6,
            i_q_max=15e-3,
            r_refs=[200e3] * 3 + [140e3] + [120e3] * 3 + [110e3],
        )
        self.assertLess(r_lim_min, 2.2e3)
        self.assertGreater(r_lim_max, 2.2e3)

    def test_elements(self):
        r_lim_min, r_lim_max = calculate(
            v_in=-12,
            v_ref=-10,
            i_q_min=45e-6,
            i_q_max=15e-3,
            r_refs=[180e3] + [120e3] * 11,
        )
        self.assertLess(r_lim_min, 500)
        self.assertGreater(r_lim_max, 500)

    def test_tides(self):
        r_lim_min, r_lim_max = calculate(
            v_in=-12,
            v_ref=-10,
            i_q_min=45e-6,
            i_q_max=15e-3,
            r_refs=[120e3] * 5 + [140e3] + [390e3] * 2,
        )
        self.assertLess(r_lim_min, 1.5e3)
        self.assertGreater(r_lim_max, 1.5e3)

    def test_clouds(self):
        r_lim_min, r_lim_max = calculate(
            v_in=-12,
            v_ref=-10,
            i_q_min=45e-6,
            i_q_max=15e-3,
            r_refs=[210e3] * 2 + [180e3] + [200e3] * 3,
        )
        self.assertLess(r_lim_min, 470)
        self.assertGreater(r_lim_max, 470)


if __name__ == "__main__":
    main()
