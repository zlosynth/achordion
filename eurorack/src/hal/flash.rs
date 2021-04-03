use super::pac::{flash, FLASH};

pub trait FlashConstrain {
    fn constrain(self) -> Parts;
}

impl FlashConstrain for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR { _0: () },
        }
    }
}

pub struct Parts {
    pub acr: ACR,
}

pub struct ACR {
    _0: (),
}

impl ACR {
    pub fn acr(&mut self) -> &flash::ACR {
        unsafe { &(*FLASH::ptr()).acr }
    }
}
