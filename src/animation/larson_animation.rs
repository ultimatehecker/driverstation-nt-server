use crate::color::RGBWColor;

pub enum LarsonBounceMode {
    FRONT,
    CENTER,
    BACK
}

pub struct LarsonAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub size: u8,
    pub bounce_mode: LarsonBounceMode,
    pub frame_rate: f64
}

impl LarsonAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, size: u8, bounce_mode: LarsonBounceMode, frame_rate: f64) -> LarsonAnimation {
        self::LarsonAnimation {
            start_index,
            end_index,
            color,
            size,
            bounce_mode,
            frame_rate
        }
    }
}