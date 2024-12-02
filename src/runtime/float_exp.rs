pub trait FloatExp {
    const NAN_CANONICAL: Self;
    const NAN_ARITHMETIC: Self;

    fn is_nan_canonical(&self) -> bool;
    fn is_nan_arithmetic(&self) -> bool;
}
impl FloatExp for f32 {
    const NAN_CANONICAL: Self = f32::from_bits(0b01000000000000000000000000000000);
    const NAN_ARITHMETIC: Self = f32::from_bits(0b01110000000000000000000000000000);

    fn is_nan_canonical(&self) -> bool {
        (0b01000000000000000000000000000000 & self.to_bits()) != 0
            && (0b0011111100000000000000000000000 & self.to_bits()) == 0
    }

    fn is_nan_arithmetic(&self) -> bool {
        (0b01000000000000000000000000000000 & self.to_bits()) != 0
    }
}

impl FloatExp for f64 {
    const NAN_CANONICAL: Self =
        f64::from_bits(0b01000000000000000000000000000000000000000000000000000000000000000);
    const NAN_ARITHMETIC: Self =
        f64::from_bits(0b01110000000000000000000000000000000000000000000000000000000000000);

    fn is_nan_canonical(&self) -> bool {
        (0b01000000000000000000000000000000000000000000000000000000000000000 & self.to_bits()) != 0
            && (0b00111111111100000000000000000000000000000000000000000000000000000
                & self.to_bits())
                == 0
    }

    fn is_nan_arithmetic(&self) -> bool {
        (0b01000000000000000000000000000000000000000000000000000000000000000 & self.to_bits()) != 0
    }
}
