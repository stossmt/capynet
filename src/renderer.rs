use crate::error::CapyError;

pub fn render_text(
    bitmap: &mut [u8],
    text: &str,
    x: usize,
    y: usize,
    window_width: usize,
    scale: usize, // Add scale parameter
) -> Result<(), CapyError> {
    let mut x_pos = x;
    for (_, char) in text.chars().enumerate() {
        let char_width = render_char(bitmap, char, x_pos, y, window_width, scale)?;
        x_pos += char_width * scale + scale; // Add scale for spacing between characters
    }
    Ok(())
}

fn render_char(
    bitmap: &mut [u8],
    char: char,
    x: usize,
    y: usize,
    window_width: usize,
    scale: usize,
) -> Result<usize, CapyError> {
    let glyph = map_char_to_glyph(char)?;
    let mut char_width = 0;
    let supersample_scale = 4; // Supersample scale factor
    let supersample_size = 8 * supersample_scale;
    let mut supersample_bitmap = vec![0u8; supersample_size * supersample_size];

    // Render the character at a higher resolution
    for (row_index, row) in glyph.iter().enumerate() {
        for col_index in 0..8 {
            let bit = (row >> (7 - col_index)) & 1;
            if bit == 1 {
                char_width = char_width.max(col_index + 1); // Update char_width to the rightmost bit set
            }
            for i in 0..supersample_scale {
                for j in 0..supersample_scale {
                    let pixel_x = col_index * supersample_scale + i;
                    let pixel_y = row_index * supersample_scale + j;
                    supersample_bitmap[pixel_y * supersample_size + pixel_x] = bit;
                }
            }
        }
    }

    // Downscale the supersampled bitmap to the target scale
    for row_index in 0..8 {
        for col_index in 0..8 {
            let mut sum = 0;
            for i in 0..supersample_scale {
                for j in 0..supersample_scale {
                    let pixel_x = col_index * supersample_scale + i;
                    let pixel_y = row_index * supersample_scale + j;
                    sum += supersample_bitmap[pixel_y * supersample_size + pixel_x] as u32;
                }
            }
            let avg = sum / (supersample_scale * supersample_scale) as u32;
            let color = 255 * (1 - avg);
            for i in 0..scale {
                for j in 0..scale {
                    let pixel_x = x + col_index * scale + i;
                    let pixel_y = y + row_index * scale + j;
                    let offset = (pixel_y * window_width + pixel_x) * 4;
                    bitmap[offset] = color as u8;
                    bitmap[offset + 1] = color as u8;
                    bitmap[offset + 2] = color as u8;
                    bitmap[offset + 3] = 255; // Alpha channel remains the same
                }
            }
        }
    }

    Ok(char_width)
}

fn map_char_to_glyph(char: char) -> Result<[u8; 8], CapyError> {
    match char {
        #[rustfmt::skip]
        'a' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b00000100,
            0b00111100,
            0b01000100,
            0b00111100,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'b' => Ok([
            0b00000000,
            0b01000000,
            0b01000000,
            0b01111000,
            0b01000100,
            0b01000100,
            0b01111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'c' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b01000000,
            0b01000000,
            0b01000000,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'd' => Ok([
            0b00000000,
            0b00001000,
            0b00001000,
            0b00111000,
            0b01001000,
            0b01001000,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'e' => Ok([
            0b00000000,
            0b00000000,
            0b00110000,
            0b01001000,
            0b01111000,
            0b01000000,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'f' => Ok([
            0b00000000,
            0b00011000,
            0b00100000,
            0b01110000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'g' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b01001000,
            0b00111000,
            0b00001000,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'h' => Ok([
            0b00000000,
            0b01000000,
            0b01000000,
            0b01110000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'i' => Ok([
            0b00000000,
            0b00100000,
            0b00000000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'j' => Ok([
            0b00000000,
            0b00010000,
            0b00000000,
            0b00010000,
            0b00010000,
            0b00010000,
            0b01100000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'k' => Ok([
            0b00000000,
            0b01000000,
            0b01000000,
            0b01001000,
            0b01110000,
            0b01001000,
            0b01001000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'l' => Ok([
            0b00000000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'm' => Ok([
            0b00000000,
            0b00000000,
            0b01101000,
            0b01010100,
            0b01010100,
            0b01010100,
            0b01010100,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'n' => Ok([
            0b00000000,
            0b00000000,
            0b01110000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'o' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b01000100,
            0b01000100,
            0b01000100,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'p' => Ok([
            0b00000000,
            0b00000000,
            0b01110000,
            0b01001000,
            0b01110000,
            0b01000000,
            0b01000000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'q' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b01001000,
            0b00111000,
            0b00001000,
            0b00001000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'r' => Ok([
            0b00000000,
            0b00000000,
            0b01110000,
            0b01001000,
            0b01000000,
            0b01000000,
            0b01000000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        's' => Ok([
            0b00000000,
            0b00000000,
            0b00111000,
            0b01000000,
            0b00111000,
            0b00001000,
            0b01110000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        't' => Ok([
            0b00000000,
            0b00100000,
            0b01110000,
            0b00100000,
            0b00100000,
            0b00100000,
            0b00011000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'u' => Ok([
            0b00000000,
            0b00000000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b00111000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'v' => Ok([
            0b00000000,
            0b00000000,
            0b01001000,
            0b01001000,
            0b01001000,
            0b00110000,
            0b00110000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'w' => Ok([
            0b00000000,
            0b00000000,
            0b01000100,
            0b01000100,
            0b01010100,
            0b01010100,
            0b00101000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'x' => Ok([
            0b00000000,
            0b00000000,
            0b01001000,
            0b00110000,
            0b00110000,
            0b01001000,
            0b01001000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'y' => Ok([
            0b00000000,
            0b00000000,
            0b01001000,
            0b01001000,
            0b00111000,
            0b00001000,
            0b01110000,
            0b00000000,
        ]),
        #[rustfmt::skip]
        'z' => Ok([
            0b00000000,
            0b00000000,
            0b01111000,
            0b00010000,
            0b00100000,
            0b01000000,
            0b01111000,
            0b00000000,
        ]),
        _ => Err(CapyError::new(
            crate::error::ErrorCode::Unknown,
            "Unknown character",
        )),
    }
}
