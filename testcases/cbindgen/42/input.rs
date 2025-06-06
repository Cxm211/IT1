#[repr(C)]
pub struct FixedPoint<const FRACTION_BITS: u16> {
    value: u16,
}

pub const FONT_WEIGHT_FRACTION_BITS: u16 = 6;

#[repr(C)]
pub struct FontWeight(FixedPoint<FONT_WEIGHT_FRACTION_BITS>);

impl FontWeight {
    pub const NORMAL: FontWeight = FontWeight(FixedPoint { value: 400 << FONT_WEIGHT_FRACTION_BITS });
}

#[no_mangle]
pub extern "C" fn root(w: FontWeight) {}