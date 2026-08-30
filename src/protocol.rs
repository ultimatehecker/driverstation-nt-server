use crate::{animation::{animation::Animation, solid_animation::SolidAnimation}, color::RGBWColor};

pub const BUFFER_ARRAY_SIZE: usize = 22;

pub fn encode(animation: &Animation, buffer: &mut [u8]) -> std::result::Result<usize, ()> {
    let mut i = 0;

    match animation {
        Animation::Solid(solid) => {
            buffer[i] = 0;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&solid.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&solid.end_index.to_le_bytes());
            i += 2;

            buffer[i] = solid.color.red;
            i += 1;

            buffer[i] = solid.color.green;
            i += 1;

            buffer[i] = solid.color.blue;
            i += 1;

            buffer[i] = solid.color.white;
            i += 1;
        }

        Animation::Strobe(strobe) => {
            buffer[i] = 1;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&strobe.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&strobe.end_index.to_le_bytes());
            i += 2;

            buffer[i] = strobe.color.red;
            i += 1;

            buffer[i] = strobe.color.green;
            i += 1;

            buffer[i] = strobe.color.blue;
            i += 1;

            buffer[i] = strobe.color.white;
            i += 1;

            buffer[i..i + 8].copy_from_slice(&strobe.frame_rate.to_le_bytes());
            i += 8;
        }

        Animation::Larson(larson) => {
            buffer[i] = 2;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&larson.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&larson.end_index.to_le_bytes());
            i += 2;

            buffer[i] = larson.color.red;
            i += 1;

            buffer[i] = larson.color.green;
            i += 1;

            buffer[i] = larson.color.blue;
            i += 1;

            buffer[i] = larson.color.white;
            i += 1;

            buffer[i..i + 8].copy_from_slice(&larson.frame_rate.to_le_bytes());
            i += 8;

            buffer[i] = larson.size;
            i += 1;

            match larson.bounce_mode {
                crate::animation::larson_animation::LarsonBounceMode::FRONT => buffer[i] = 0,
                crate::animation::larson_animation::LarsonBounceMode::CENTER => buffer[i] = 1,
                crate::animation::larson_animation::LarsonBounceMode::BACK => buffer[i] = 2,
            }

            i += 1;
        }

        Animation::Rainbow(rainbow) => {
            buffer[i] = 3;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&rainbow.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&rainbow.end_index.to_le_bytes());
            i += 2;

            buffer[i..i + 8].copy_from_slice(&rainbow.frame_rate.to_le_bytes());
            i += 8;

            buffer[i..i + 8].copy_from_slice(&rainbow.brightness.to_le_bytes());
            i += 8;

            match rainbow.direction {
                crate::animation::rainbow_animation::AnimationDirection::FORWARD => buffer[i] = 0,
                crate::animation::rainbow_animation::AnimationDirection::BACKWARD => buffer[i] = 1,
            }

            i += 1;
        }
    }

    println!("{buffer:?}");

    Ok(i)
}

pub fn main() {
    let animation = Animation::Solid(SolidAnimation::new(0, 30, RGBWColor::new(0, 0, 0, 255)));
    println!("{animation:?}");

    let mut bytes = [0u8; BUFFER_ARRAY_SIZE];
    encode(&animation, &mut bytes);
}