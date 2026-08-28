#[derive(Debug)]
struct RGBColor {
    red: u8,
    green: u8,
    blue: u8
}

#[derive(Debug)]
struct RGBWColor {
    red: u8,
    green: u8,
    blue: u8,
    white: u8
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
}