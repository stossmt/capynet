use crate::error::CapyError;

pub fn render_text(
    bitmap: &mut [u8],
    text: &str,
    x: usize,
    y: usize,
    window_width: usize,
    scale: usize, // Add scale parameter
) -> Result<(), CapyError> {
    let glyph = map_char_to_glyph('a')?;
    for (row_index, row) in glyph.iter().enumerate() {
        for col_index in 0..8 {
            let bit = (row >> col_index) & 1;
            for i in 0..scale {
                for j in 0..scale {
                    let pixel_x = x + col_index * scale + i;
                    let pixel_y = y + row_index * scale + j;
                    let offset = (pixel_y * window_width + pixel_x) * 4;
                    let color = 255 * (1 - bit); // Invert bit to draw text in black
                    bitmap[offset] = color;
                    bitmap[offset + 1] = color;
                    bitmap[offset + 2] = color;
                    bitmap[offset + 3] = 255; // Alpha channel remains the same
                }
            }
        }
    }
    Ok(())
}

fn map_char_to_glyph(char: char) -> Result<[u8; 8], CapyError> {
    match char {
        #[rustfmt::skip]
        'a' => Ok([
            0b00011000,
            0b00100100,
            0b01000010,
            0b01000010,
            0b01111110,
            0b01000010,
            0b01000010,
            0b01000010,
        ]),
        _ => Err(CapyError::new(
            crate::error::ErrorCode::Unknown,
            "Unknown character",
        )),
    }
}
