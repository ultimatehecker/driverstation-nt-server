use crate::color::RGBWColor;

pub enum AnimationMode {
    FORWARD,
    BACKWARD
}

pub struct RainbowAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub brightness: f64,
    pub animation_mode: AnimationMode,
    pub frame_rate: f64
}

impl RainbowAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, brightness: f64, animation_mode: AnimationMode, frame_rate: f64) -> RainbowAnimation {
        self::RainbowAnimation {
            start_index,
            end_index,
            color,
            brightness,
            animation_mode,
            frame_rate
        }
    }
}