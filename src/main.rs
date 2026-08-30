#[derive(Debug, PartialEq)]
struct RGBColor {
    red: u8,
    green: u8,
    blue: u8
}

#[derive(Debug, PartialEq)]
struct RGBWColor {
    red: u8,
    green: u8,
    blue: u8,
    white: u8
}

#[derive(Debug, PartialEq)]
struct SolidColorAnimation {
    start_index: u16,
    end_index: u16,
    color: RGBColor
}

struct StrobeAnimation {
    start_index: u16,
    end_index: u16,
    color: RGBColor,
    frame_rate: f64
}

#[derive(Debug, PartialEq)]
struct LarsonAnimation {
    start_index: u16,
    end_index: u16,
    color: RGBColor,
    size: u8,
    bounce_mode: LarsonBounceMode,
    frame_rate: f64
}

#[derive(Debug, PartialEq)]
struct RainbowAnimation {
    start_index: u16,
    end_index: u16,
    color: RGBColor,
    brightness: f64,
    direction: AnimationDirection,
    frame_rate: f64
}

#[derive(Debug, PartialEq)]
enum AnimationDirection {
    FORWARD,
    BACKWARDS
}

#[derive(Debug, PartialEq)]
enum LarsonBounceMode {
    FRONT,
    CENTER,
    BACK
}

impl RGBColor {
    fn new(red: u8, green: u8, blue: u8) -> RGBColor {
        self::RGBColor {
            red,
            green,
            blue
        }
    }
}

impl RGBWColor {
    fn new(red: u8, green: u8, blue: u8, white: u8) -> RGBWColor {
        self::RGBWColor {
            red,
            green,
            blue,
            white
        }
    }
}

impl SolidColorAnimation {
    fn new(start_index: u16, end_index: u16, color: RGBColor) -> SolidColorAnimation {
        self::SolidColorAnimation {
            start_index,
            end_index,
            color
        }
    }
}

impl StrobeAnimation {
    fn new(start_index: u16, end_index: u16, color: RGBColor, frame_rate: f64) -> StrobeAnimation {
        self::StrobeAnimation {
            start_index,
            end_index,
            color,
            frame_rate
        }
    }
}

impl LarsonAnimation {
    fn new(start_index: u16, end_index: u16, color: RGBColor, size: u8, bounce_mode: LarsonBounceMode, frame_rate: f64) -> LarsonAnimation {
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

impl RainbowAnimation {
    fn new(start_index: u16, end_index: u16, color: RGBColor, brightness: f64, direction: AnimationDirection, frame_rate: f64) -> RainbowAnimation {
        self::RainbowAnimation {
            start_index,
            end_index,
            color,
            brightness,
            direction,
            frame_rate
        }
    }
}

fn main() {
    
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn instantize_rgb_color() {
        let rgb_color = RGBColor::new(255, 255, 255);

        assert_eq!(rgb_color.red, 255);
        assert_eq!(rgb_color.green, 255);
        assert_eq!(rgb_color.blue, 255);
    }

    #[test]
    fn instantize_rgbw_color() {
        let rgbw_color = RGBWColor::new(255, 255, 255, 255);

        assert_eq!(rgbw_color.red, 255);
        assert_eq!(rgbw_color.green, 255);
        assert_eq!(rgbw_color.blue, 255);
        assert_eq!(rgbw_color.white, 255);
    }

    #[test]
    fn instantize_animation_solid_color() {
        let animation: SolidColorAnimation = SolidColorAnimation::new(0, 89, RGBColor::new(255, 10, 135));

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBColor::new(255, 10, 135));
    }

    #[test]
    fn instantize_animation_strobe() {
        let animation: StrobeAnimation = StrobeAnimation::new(0, 89, RGBColor::new(255, 10, 135), 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBColor::new(255, 10, 135));
        assert_eq!(animation.frame_rate, 60.0);
    }

    #[test]
    fn instantize_animation_larson() {
        let animation: LarsonAnimation = LarsonAnimation::new(0, 89, RGBColor::new(255, 10, 135), 10, LarsonBounceMode::FRONT, 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBColor::new(255, 10, 135));
        assert_eq!(animation.size, 10);
        assert_eq!(animation.bounce_mode, LarsonBounceMode::FRONT);
        assert_eq!(animation.frame_rate, 60.0)

    }

    #[test]
    fn instantize_animation_rainbow() {
        let animation: RainbowAnimation = RainbowAnimation::new(0, 89, RGBColor::new(255, 10, 135), 1.0, AnimationDirection::BACKWARDS, 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBColor::new(255, 10, 135));
        assert_eq!(animation.brightness, 1.0);
        assert_eq!(animation.direction, AnimationDirection::BACKWARDS);
        assert_eq!(animation.frame_rate, 60.0);
    }
}