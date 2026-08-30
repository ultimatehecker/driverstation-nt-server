use crate::color::RGBWColor;

pub struct StrobeAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub frame_rate: f64
}

impl StrobeAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, frame_rate: f64) -> StrobeAnimation {
        self::StrobeAnimation {
            start_index,
            end_index,
            color,
            frame_rate
        }
    }
}