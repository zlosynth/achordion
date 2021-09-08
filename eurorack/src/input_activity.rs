pub struct InputActivity {
    pot_idle: u32,
    cv_idle: u32,
}

impl InputActivity {
    const MAX_IDLE_POT: u32 = 1200;
    const MAX_IDLE_CV: u32 = 3200;

    pub fn new() -> Self {
        Self {
            pot_idle: u32::MAX,
            cv_idle: u32::MAX,
        }
    }

    pub fn reset_pots(&mut self) {
        self.pot_idle = 0;
    }

    pub fn reset_cv(&mut self) {
        self.cv_idle = 0;
    }

    pub fn tick_all(&mut self) {
        self.pot_idle += 1;
        self.cv_idle += 1;
    }

    pub fn idle_pots(&self) -> bool {
        self.pot_idle > Self::MAX_IDLE_POT
    }

    pub fn idle_cv(&self) -> bool {
        self.cv_idle > Self::MAX_IDLE_CV
    }
}
