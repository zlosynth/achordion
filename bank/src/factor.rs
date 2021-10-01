pub type Factor1024 = [u16; 1024];
pub type Factor512 = [u16; 512];
pub type Factor256 = [u16; 256];
pub type Factor128 = [u16; 128];
pub type Factor64 = [u16; 64];
pub type Factor32 = Factor64;
pub type Factor16 = Factor64;
pub type Factor8 = Factor64;
pub type Factor4 = Factor64;
pub type Factor2 = Factor64;
pub type Factor1 = Factor64;

pub struct Factors {
    pub factor1024: Factor1024,
    pub factor512: Factor512,
    pub factor256: Factor256,
    pub factor128: Factor128,
    pub factor64: Factor64,
    pub factor32: Factor32,
    pub factor16: Factor16,
    pub factor8: Factor8,
    pub factor4: Factor4,
    pub factor2: Factor2,
    pub factor1: Factor1,
}

pub struct FactorsRef<'a>([&'a [u16]; 11]);

impl<'a> FactorsRef<'a> {
    pub fn new(factors: &'a Factors) -> Self {
        Self([
            &factors.factor1024,
            &factors.factor512,
            &factors.factor256,
            &factors.factor128,
            &factors.factor64,
            &factors.factor32,
            &factors.factor16,
            &factors.factor8,
            &factors.factor4,
            &factors.factor2,
            &factors.factor1,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const F1024: Factor1024 = [0; 1024];
    const F512: Factor512 = [0; 512];
    const F256: Factor256 = [0; 256];
    const F128: Factor128 = [0; 128];
    const F64: Factor64 = [0; 64];
    const F32: Factor32 = [0; 64];
    const F16: Factor16 = [0; 64];
    const F8: Factor8 = [0; 64];
    const F4: Factor4 = [0; 64];
    const F2: Factor2 = [0; 64];
    const F1: Factor1 = [0; 64];
    const FS: Factors = Factors {
        factor1024: F1024,
        factor512: F512,
        factor256: F256,
        factor128: F128,
        factor64: F64,
        factor32: F32,
        factor16: F16,
        factor8: F8,
        factor4: F4,
        factor2: F2,
        factor1: F1,
    };

    #[test]
    fn initialize_factors_ref() {
        let _reference = FactorsRef::new(&FS);
    }
}
