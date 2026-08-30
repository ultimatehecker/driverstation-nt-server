use crate::color::RGBWColor;

pub struct SolidAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor
}

impl SolidAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor) -> SolidAnimation {
        self::SolidAnimation {
            start_index,
            end_index,
            color
        }
    }
}