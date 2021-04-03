use core::convert::TryInto;

use embedded_time::rate::*;

use super::flash::ACR;

use super::pac::{
    rcc::{self, cfgr, cfgr2},
    RCC,
};

const HSI: u32 = 8_000_000;

pub trait RccConstrain {
    fn constrain(self) -> Rcc;
}

impl RccConstrain for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb: AHB { _0: () },
            apb1: APB1 { _0: () },
            apb2: APB2 { _0: () },
            cfgr: CFGR::default(),
        }
    }
}

pub struct Rcc {
    /// AMBA High-performance Bus (AHB) registers
    pub ahb: AHB,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb1: APB1,
    /// Advanced Peripheral Bus 1 (APB1) registers
    pub apb2: APB2,
    /// Clock configuration
    pub cfgr: CFGR,
}

/// AMBA High-performance Bus (AHB) registers
pub struct AHB {
    _0: (),
}

impl AHB {
    pub fn enr(&mut self) -> &rcc::AHBENR {
        unsafe { &(*RCC::ptr()).ahbenr }
    }

    pub fn rstr(&mut self) -> &rcc::AHBRSTR {
        unsafe { &(*RCC::ptr()).ahbrstr }
    }
}

/// Advanced Peripheral Bus 1 (APB1) registers
pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub fn enr(&mut self) -> &rcc::APB1ENR {
        unsafe { &(*RCC::ptr()).apb1enr }
    }
}

/// Advanced Peripheral Bus 2 (APB2) registers
pub struct APB2 {
    _0: (),
}

impl APB2 {
    pub fn enr(&mut self) -> &rcc::APB2ENR {
        unsafe { &(*RCC::ptr()).apb2enr }
    }
}

#[derive(Default)]
pub struct CFGR {
    hse: Option<u32>,
    hse_bypass: bool,
    css: bool,
    hclk: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    sysclk: Option<u32>,
}

pub struct PllConfig {
    src: cfgr::PLLSRC_A,
    mul: cfgr::PLLMUL_A,
    div: Option<cfgr2::PREDIV_A>,
}

/// Determine the [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
///
/// This function is based on the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm).
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// Convert pll multiplier into equivalent register field type
fn into_pll_mul(mul: u8) -> cfgr::PLLMUL_A {
    match mul {
        2 => cfgr::PLLMUL_A::MUL2,
        3 => cfgr::PLLMUL_A::MUL3,
        4 => cfgr::PLLMUL_A::MUL4,
        5 => cfgr::PLLMUL_A::MUL5,
        6 => cfgr::PLLMUL_A::MUL6,
        7 => cfgr::PLLMUL_A::MUL7,
        8 => cfgr::PLLMUL_A::MUL8,
        9 => cfgr::PLLMUL_A::MUL9,
        10 => cfgr::PLLMUL_A::MUL10,
        11 => cfgr::PLLMUL_A::MUL11,
        12 => cfgr::PLLMUL_A::MUL12,
        13 => cfgr::PLLMUL_A::MUL13,
        14 => cfgr::PLLMUL_A::MUL14,
        15 => cfgr::PLLMUL_A::MUL15,
        16 => cfgr::PLLMUL_A::MUL16,
        _ => unreachable!(),
    }
}

/// Convert pll divisor into equivalent register field type
fn into_pre_div(div: u8) -> cfgr2::PREDIV_A {
    match div {
        1 => cfgr2::PREDIV_A::DIV1,
        2 => cfgr2::PREDIV_A::DIV2,
        3 => cfgr2::PREDIV_A::DIV3,
        4 => cfgr2::PREDIV_A::DIV4,
        5 => cfgr2::PREDIV_A::DIV5,
        6 => cfgr2::PREDIV_A::DIV6,
        7 => cfgr2::PREDIV_A::DIV7,
        8 => cfgr2::PREDIV_A::DIV8,
        9 => cfgr2::PREDIV_A::DIV9,
        10 => cfgr2::PREDIV_A::DIV10,
        11 => cfgr2::PREDIV_A::DIV11,
        12 => cfgr2::PREDIV_A::DIV12,
        13 => cfgr2::PREDIV_A::DIV13,
        14 => cfgr2::PREDIV_A::DIV14,
        15 => cfgr2::PREDIV_A::DIV15,
        16 => cfgr2::PREDIV_A::DIV16,
        _ => unreachable!(),
    }
}

impl CFGR {
    /// Uses `HSE` (external oscillator) instead of `HSI` (internal RC oscillator) as the clock source.
    ///
    /// Will result in a hang if an external oscillator is not connected or it fails to start,
    /// unless [css](CFGR::enable_css) is enabled.
    ///
    /// # Panics
    ///
    /// Panics if conversion from `Megahertz` to `Hertz` produces a value greater then `u32::MAX`.
    pub fn use_hse(mut self, freq: Megahertz) -> Self {
        let freq: Hertz = freq.try_into().expect("ConversionError");
        self.hse = Some(*freq.integer());
        self
    }

    /// Enable `HSE` bypass.
    ///
    /// Uses user provided clock signal instead of an external oscillator.
    /// `OSC_OUT` pin is free and can be used as GPIO.
    ///
    /// No effect if `HSE` is not enabled.
    pub fn bypass_hse(mut self) -> Self {
        self.hse_bypass = true;
        self
    }

    /// Enable `CSS` (Clock Security System).
    ///
    /// System clock is automatically switched to `HSI` and an interrupt (`CSSI`) is generated
    /// when `HSE` clock failure is detected.
    ///
    /// No effect if `HSE` is not enabled.
    pub fn enable_css(mut self) -> Self {
        self.css = true;
        self
    }

    /// Sets a frequency for the AHB bus.
    ///
    /// # Panics
    ///
    /// Panics if conversion from `Megahertz` to `Hertz` produces a value greater then `u32::MAX`.
    pub fn hclk(mut self, freq: Megahertz) -> Self {
        let freq: Hertz = freq.try_into().expect("ConversionError");
        self.hclk = Some(*freq.integer());
        self
    }

    /// Sets a frequency for the `APB1` bus
    ///
    /// - Maximal supported frequency: 36 Mhz
    ///
    /// If not manually set, it will be set to [`CFGR::sysclk`] frequency
    /// or [`CFGR::sysclk`] frequency / 2, if [`CFGR::sysclk`] > 36 Mhz
    ///
    /// # Panics
    ///
    /// Panics if conversion from `Megahertz` to `Hertz` produces a value greater then `u32::MAX`.
    pub fn pclk1(mut self, freq: Megahertz) -> Self {
        let freq: Hertz = freq.try_into().expect("ConversionError");
        self.pclk1 = Some(*freq.integer());
        self
    }

    /// Sets a frequency for the `APB2` bus
    ///
    /// # Resolution and Limits
    ///
    /// - Maximal supported frequency with HSE: 72 Mhz
    /// - Maximal supported frequency without HSE: 64 Mhz
    ///
    /// This is true for devices **except** the following devices,
    /// as these allow finer resolutions
    /// even when using the internal oscillator:
    ///
    ///     [stm32f302xd,stm32f302xe,stm32f303xd,stm32f303xe,stm32f398]
    ///
    /// # Panics
    ///
    /// Panics if conversion from `Megahertz` to `Hertz` produces a value greater then `u32::MAX`.
    pub fn pclk2(mut self, freq: Megahertz) -> Self {
        let freq: Hertz = freq.try_into().expect("ConversionError");
        self.pclk2 = Some(*freq.integer());
        self
    }

    /// Sets the system (core) frequency
    ///
    /// # Resolution and Limits
    ///
    /// - Maximal supported frequency with `HSE`: 72 Mhz
    /// - Maximal supported frequency without `HSE`: 64 Mhz
    ///
    /// If [`CFGR::hse`] is not used, therefor `HSI / 2` is used.
    /// Only multiples of (HSI / 2) (4 Mhz) are allowed.
    ///
    /// This is true for devices **except** the following devices,
    /// as these allow finer resolutions
    /// even when using the internal oscillator:
    ///
    ///     [stm32f302xd,stm32f302xe,stm32f303xd,stm32f303xe,stm32f398]
    ///
    /// # Panics
    ///
    /// Panics if conversion from `Megahertz` to `Hertz` produces a value greater then `u32::MAX`.
    pub fn sysclk(mut self, freq: Megahertz) -> Self {
        let freq: Hertz = freq.try_into().expect("ConversionError");
        self.sysclk = Some(*freq.integer());
        self
    }

    /// Calculate the values for the pll multiplier (`PLLMUL`) and the pll divisior (`PLLDIV`).
    ///
    /// These values are chosen depending on the chosen system clock (SYSCLK) and the frequency of the
    /// oscillator clock (`HSE` / `HSI`).
    ///
    /// For these devices, `PLL_SRC` can selected between the internal oscillator (`HSI`) and
    /// the external oscillator (`HSE`).
    ///
    /// HSI is divided by 2 before its transferred to `PLL_SRC`.
    /// HSE can be divided between `1..16`, before it is transferred to `PLL_SRC`.
    /// After this system clock frequency (`SYSCLK`) can be changed via multiplier.
    /// The value can be multiplied with `2..16`.
    ///
    /// To determine the optimal values, if `HSE` is chosen as `PLL_SRC`, the greatest common divisor
    /// is calculated and the limitations of the possible values are taken into consideration.
    ///
    /// `HSI` is simpler to calculate, but the possible system clocks are less than `HSE`, because the
    /// division is not configurable.
    fn calc_pll(&self, sysclk: u32) -> (u32, PllConfig) {
        let pllsrcclk = self.hse.unwrap_or(HSI / 2);
        // Get the optimal value for the pll divisor (PLL_DIV) and multiplier (PLL_MUL)
        // Only for HSE PLL_DIV can be changed
        let (pll_mul, pll_div): (u32, Option<u32>) = if self.hse.is_some() {
            // Get the optimal value for the pll divisor (PLL_DIV) and multiplier (PLL_MUL)
            // with the greatest common divisor calculation.
            let common_divisor = gcd(sysclk, pllsrcclk);
            let mut multiplier = sysclk / common_divisor;
            let mut divisor = pllsrcclk / common_divisor;

            // Check if the multiplier can be represented by PLL_MUL
            if multiplier == 1 {
                // PLL_MUL minimal value is 2
                multiplier *= 2;
                divisor *= 2;
            }

            // PLL_MUL maximal value is 16
            assert!(multiplier <= 16);

            // PRE_DIV maximal value is 16
            assert!(divisor <= 16);

            (multiplier, Some(divisor))
        }
        // HSI division is always divided by 2 and has no adjustable division
        else {
            let pll_mul = sysclk / pllsrcclk;
            assert!(pll_mul <= 16);
            (pll_mul, None)
        };

        let sysclk = (pllsrcclk / pll_div.unwrap_or(1)) * pll_mul;
        assert!(sysclk <= 72_000_000);

        let pll_src = if self.hse.is_some() {
            cfgr::PLLSRC_A::HSE_DIV_PREDIV
        } else {
            cfgr::PLLSRC_A::HSI_DIV2
        };

        // Convert into register bit field types
        let pll_mul_bits = into_pll_mul(pll_mul as u8);
        let pll_div_bits = pll_div.map(|pll_div| into_pre_div(pll_div as u8));

        (
            sysclk,
            PllConfig {
                src: pll_src,
                mul: pll_mul_bits,
                div: pll_div_bits,
            },
        )
    }

    /// Get the system clock, the system clock source and the pll_options, if needed.
    ///
    /// The system clock source is determined by the chosen system clock and the provided hardware
    /// clock.
    /// This function does only chose the PLL if needed, otherwise it will use the oscillator clock as system clock.
    ///
    /// Calls [`CFGR::calc_pll`] internally.
    fn get_sysclk(&self) -> (u32, cfgr::SW_A, Option<PllConfig>) {
        // If a sysclk is given, check if the PLL has to be used,
        // else select the system clock source, which is either HSI or HSE.
        match (self.sysclk, self.hse) {
            // No need to use the PLL
            // PLL is needed for USB, but we can make this assumption, to not use PLL here,
            // because the two valid USB clocks, 72 Mhz and 48 Mhz, can't be generated
            // directly from neither the internal rc (8 Mhz)  nor the external
            // Oscillator (max 32 Mhz), without using the PLL.
            (Some(sysclk), Some(hse)) if sysclk == hse => (hse, cfgr::SW_A::HSE, None),
            // No need to use the PLL
            (Some(sysclk), None) if sysclk == HSI => (HSI, cfgr::SW_A::HSI, None),
            (Some(sysclk), _) => {
                let (sysclk, pll_config) = self.calc_pll(sysclk);
                (sysclk, cfgr::SW_A::PLL, Some(pll_config))
            }
            // Use HSE as system clock
            (None, Some(hse)) => (hse, cfgr::SW_A::HSE, None),
            // Use HSI as system clock
            (None, None) => (HSI, cfgr::SW_A::HSI, None),
        }
    }

    /// Freezes the clock configuration, making it effective
    ///
    /// This function internally calculates the specific.
    /// divisors for the different clock peripheries.
    ///
    /// # Panics
    ///
    /// If any of the set frequencies via [`sysclk`](CFGR::sysclk), [`hclk`](CFGR::hclk), [`pclk1`](CFGR::pclk1) or [`pclk2`](CFGR::pclk2)
    /// are invalid or can not be reached because of e.g. to low frequencies
    /// of the former, as [`sysclk`](CFGR::sysclk) depends on the configuration of [`hclk`](CFGR::hclk)
    /// this function will panic.
    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        let (sysclk, sysclk_source, pll_config) = self.get_sysclk();

        let (hpre_bits, hpre) =
            self.hclk
                .map_or((cfgr::HPRE_A::DIV1, 1), |hclk| match sysclk / hclk {
                    0 => unreachable!(),
                    1 => (cfgr::HPRE_A::DIV1, 1),
                    2 => (cfgr::HPRE_A::DIV2, 2),
                    3..=5 => (cfgr::HPRE_A::DIV4, 4),
                    6..=11 => (cfgr::HPRE_A::DIV8, 8),
                    12..=39 => (cfgr::HPRE_A::DIV16, 16),
                    40..=95 => (cfgr::HPRE_A::DIV64, 64),
                    96..=191 => (cfgr::HPRE_A::DIV128, 128),
                    192..=383 => (cfgr::HPRE_A::DIV256, 256),
                    _ => (cfgr::HPRE_A::DIV512, 512),
                });

        let hclk: u32 = sysclk / hpre;

        assert!(hclk <= 72_000_000);

        let (mut ppre1_bits, mut ppre1) =
            self.pclk1
                .map_or((cfgr::PPRE1_A::DIV1, 1), |pclk1| match hclk / pclk1 {
                    0 => unreachable!(),
                    1 => (cfgr::PPRE1_A::DIV1, 1),
                    2 => (cfgr::PPRE1_A::DIV2, 2),
                    3..=5 => (cfgr::PPRE1_A::DIV4, 4),
                    6..=11 => (cfgr::PPRE1_A::DIV8, 8),
                    _ => (cfgr::PPRE1_A::DIV16, 16),
                });

        let mut pclk1 = hclk / u32::from(ppre1);

        // This ensures, that no panic happens, when
        // pclk1 is not manually set.
        // As hclk highest value is 72.MHz()
        // dividing by 2 should always be sufficient
        if self.pclk1.is_none() && pclk1 > 36_000_000 {
            ppre1_bits = cfgr::PPRE1_A::DIV2;
            ppre1 = 2;
            pclk1 = hclk / u32::from(ppre1);
        }

        assert!(pclk1 <= 36_000_000);

        let (ppre2_bits, ppre2) =
            self.pclk2
                .map_or((cfgr::PPRE2_A::DIV1, 1), |pclk2| match hclk / pclk2 {
                    0 => unreachable!(),
                    1 => (cfgr::PPRE2_A::DIV1, 1),
                    2 => (cfgr::PPRE2_A::DIV2, 2),
                    3..=5 => (cfgr::PPRE2_A::DIV4, 4),
                    6..=11 => (cfgr::PPRE2_A::DIV8, 8),
                    _ => (cfgr::PPRE2_A::DIV16, 16),
                });

        let pclk2 = hclk / u32::from(ppre2);

        assert!(pclk2 <= 72_000_000);

        // Adjust flash wait states according to the
        // HCLK frequency (cpu core clock)
        acr.acr().modify(|_, w| {
            if hclk <= 24_000_000 {
                w.latency().ws0()
            } else if hclk <= 48_000_000 {
                w.latency().ws1()
            } else {
                w.latency().ws2()
            }
        });

        let (usbpre, usbclk_valid) = usb_clocking::is_valid(sysclk, self.hse, pclk1, &pll_config);

        let rcc = unsafe { &*RCC::ptr() };

        // enable HSE and wait for it to be ready
        if self.hse.is_some() {
            rcc.cr.modify(|_, w| {
                w.hsebyp().bit(self.hse_bypass);
                w.csson().bit(self.css);
                w.hseon().on()
            });

            while rcc.cr.read().hserdy().is_not_ready() {}
        }

        // enable PLL and wait for it to be ready
        if let Some(pll_config) = pll_config {
            rcc.cfgr.modify(|_, w| {
                w.pllmul()
                    .variant(pll_config.mul)
                    .pllsrc()
                    .variant(pll_config.src)
            });

            if let Some(pll_div) = pll_config.div {
                rcc.cfgr2.modify(|_, w| w.prediv().variant(pll_div));
            };

            rcc.cr.modify(|_, w| w.pllon().on());

            while rcc.cr.read().pllrdy().is_not_ready() {}
        };

        // set prescalers and clock source
        rcc.cfgr.modify(|_, w| {
            usb_clocking::set_usbpre(w, usbpre);

            w.ppre2()
                .variant(ppre2_bits)
                .ppre1()
                .variant(ppre1_bits)
                .hpre()
                .variant(hpre_bits)
                .sw()
                .variant(sysclk_source)
        });

        Clocks {
            hclk: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            ppre1,
            ppre2,
            sysclk: Hertz(sysclk),
            usbclk_valid,
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed.
/// This struct can be obtained via the [freeze](CFGR::freeze) method of the [CFGR](CFGR) struct.
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
    usbclk_valid: bool,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    /// Returns the prescaler of the APB1
    pub fn ppre1(&self) -> u8 {
        self.ppre1
    }

    /// Returns the prescaler of the APB2
    pub fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns whether the USBCLK clock frequency is valid for the USB peripheral
    ///
    /// If the microcontroller does support USB, 48 Mhz or 72 Mhz have to be used
    /// and the [`CFGR::hse`] must be used.
    ///
    /// The APB1 / [`CFGR::pclk1`] clock must have a minimum frequency of 10 MHz to avoid data
    /// overrun/underrun problems. [RM0316 32.5.2][RM0316]
    ///
    /// [RM0316]: https://www.st.com/resource/en/reference_manual/dm00043574.pdf
    pub fn usbclk_valid(&self) -> bool {
        self.usbclk_valid
    }
}

mod usb_clocking {
    use super::super::pac::rcc::cfgr;
    use super::PllConfig;

    /// Check for all clock options to be
    pub(crate) fn is_valid(
        sysclk: u32,
        hse: Option<u32>,
        pclk1: u32,
        pll_config: &Option<PllConfig>,
    ) -> (cfgr::USBPRE_A, bool) {
        // the USB clock is only valid if an external crystal is used, the PLL is enabled, and the
        // PLL output frequency is a supported one.
        // usbpre == false: divide clock by 1.5, otherwise no division
        let usb_ok = hse.is_some() && pll_config.is_some();
        // The APB1 clock must have a minimum frequency of 10 MHz to avoid data overrun/underrun
        // problems. [RM0316 32.5.2]
        if pclk1 >= 10_000_000 {
            match (usb_ok, sysclk) {
                (true, 72_000_000) => (cfgr::USBPRE_A::DIV1_5, true),
                (true, 48_000_000) => (cfgr::USBPRE_A::DIV1, true),
                _ => (cfgr::USBPRE_A::DIV1, false),
            }
        } else {
            (cfgr::USBPRE_A::DIV1, false)
        }
    }

    pub(crate) fn set_usbpre(w: &mut cfgr::W, usb_prescale: cfgr::USBPRE_A) -> &mut cfgr::W {
        w.usbpre().variant(usb_prescale)
    }
}
