#[allow(clippy::wrong_self_convention)]
pub trait FloatExp {
    type Int;
    const NAN_CANONICAL: Self;
    const NAN_ARITHMETIC: Self;

    fn is_nan_canonical(self) -> bool;
    fn is_nan_arithmetic(self) -> bool;
    fn nan_sign(self) -> Self::Int;
}
impl FloatExp for f32 {
    type Int = u32;
    const NAN_CANONICAL: Self = f32::from_bits(0b01000000000000000000000000000000);
    const NAN_ARITHMETIC: Self = f32::from_bits(0b01000000000000000000000000000000);

    fn is_nan_canonical(self) -> bool {
        (0b01000000000000000000000000000000 & self.to_bits()) != 0
            && (0b0011111110000000000000000000000 & self.to_bits()) == 0
    }

    fn is_nan_arithmetic(self) -> bool {
        (0b01000000000000000000000000000000 & self.to_bits()) != 0
    }

    fn nan_sign(self) -> Self::Int {
        self.to_bits() & 0b10000000000000000000000000000000
    }
}

impl FloatExp for f64 {
    type Int = u64;
    const NAN_CANONICAL: Self =
        f64::from_bits(0b0100000000000000000000000000000000000000000000000000000000000000);
    const NAN_ARITHMETIC: Self =
        f64::from_bits(0b0100000000000000000000000000000000000000000000000000000000000000);

    fn is_nan_canonical(self) -> bool {
        (0b0100000000000000000000000000000000000000000000000000000000000000 & self.to_bits()) != 0
            && (0b0011111111110000000000000000000000000000000000000000000000000000 & self.to_bits())
                == 0
    }

    fn is_nan_arithmetic(self) -> bool {
        (0b0100000000000000000000000000000000000000000000000000000000000000 & self.to_bits()) != 0
    }

    fn nan_sign(self) -> Self::Int {
        self.to_bits() & 0b1000000000000000000000000000000000000000000000000000000000000000
    }
}
