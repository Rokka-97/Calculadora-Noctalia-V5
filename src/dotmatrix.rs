use crate::theme::Color;

/// 5 columns x 7 rows dot matrix font for retro LCD calculator screens
pub const FONT_5X7: &[(char, [u8; 7])] = &[
    ('0', [
        0b01110,
        0b10001,
        0b10011,
        0b10101,
        0b11001,
        0b10001,
        0b01110,
    ]),
    ('1', [
        0b00100,
        0b01100,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b01110,
    ]),
    ('2', [
        0b01110,
        0b10001,
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b11111,
    ]),
    ('3', [
        0b11111,
        0b00010,
        0b00100,
        0b01110,
        0b00001,
        0b10001,
        0b01110,
    ]),
    ('4', [
        0b00010,
        0b00110,
        0b01010,
        0b10010,
        0b11111,
        0b00010,
        0b00010,
    ]),
    ('5', [
        0b11111,
        0b10000,
        0b11110,
        0b00001,
        0b00001,
        0b10001,
        0b01110,
    ]),
    ('6', [
        0b01110,
        0b10000,
        0b11110,
        0b10001,
        0b10001,
        0b10001,
        0b01110,
    ]),
    ('7', [
        0b11111,
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b01000,
        0b01000,
    ]),
    ('8', [
        0b01110,
        0b10001,
        0b10001,
        0b01110,
        0b10001,
        0b10001,
        0b01110,
    ]),
    ('9', [
        0b01110,
        0b10001,
        0b10001,
        0b01111,
        0b00001,
        0b00001,
        0b01110,
    ]),
    ('.', [
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b01100,
        0b01100,
    ]),
    (',', [
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00100,
        0b00100,
        0b01000,
    ]),
    ('+', [
        0b00000,
        0b00100,
        0b00100,
        0b11111,
        0b00100,
        0b00100,
        0b00000,
    ]),
    ('-', [
        0b00000,
        0b00000,
        0b00000,
        0b11111,
        0b00000,
        0b00000,
        0b00000,
    ]),
    ('*', [
        0b00000,
        0b10101,
        0b01110,
        0b11111,
        0b01110,
        0b10101,
        0b00000,
    ]),
    ('x', [
        0b00000,
        0b10001,
        0b01010,
        0b00100,
        0b01010,
        0b10001,
        0b00000,
    ]),
    ('/', [
        0b00001,
        0b00010,
        0b00100,
        0b01000,
        0b10000,
        0b00000,
        0b00000,
    ]),
    ('^', [
        0b00100,
        0b01010,
        0b10001,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
    ]),
    ('v', [ // Used for square root
        0b00000,
        0b00011,
        0b00100,
        0b00100,
        0b10100,
        0b01000,
        0b00000,
    ]),
    ('√', [
        0b00000,
        0b00011,
        0b00100,
        0b00100,
        0b10100,
        0b01000,
        0b00000,
    ]),
    ('%', [
        0b11001,
        0b11010,
        0b00100,
        0b01000,
        0b01011,
        0b10011,
        0b00000,
    ]),
    ('(', [
        0b00110,
        0b01000,
        0b10000,
        0b10000,
        0b10000,
        0b01000,
        0b00110,
    ]),
    (')', [
        0b01100,
        0b00010,
        0b00001,
        0b00001,
        0b00001,
        0b00010,
        0b01100,
    ]),
    ('=', [
        0b00000,
        0b11111,
        0b00000,
        0b11111,
        0b00000,
        0b00000,
        0b00000,
    ]),
    (':', [
        0b00000,
        0b01100,
        0b01100,
        0b00000,
        0b01100,
        0b01100,
        0b00000,
    ]),
    ('E', [
        0b11111,
        0b10000,
        0b11110,
        0b10000,
        0b10000,
        0b11111,
        0b00000,
    ]),
    ('R', [
        0b11110,
        0b10001,
        0b11110,
        0b10100,
        0b10010,
        0b10001,
        0b00000,
    ]),
    ('D', [
        0b11110,
        0b10001,
        0b10001,
        0b10001,
        0b10001,
        0b11110,
        0b00000,
    ]),
    ('I', [
        0b01110,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b01110,
        0b00000,
    ]),
    ('V', [
        0b10001,
        0b10001,
        0b10001,
        0b10001,
        0b01010,
        0b00100,
        0b00000,
    ]),
    ('S', [
        0b01111,
        0b10000,
        0b11110,
        0b00001,
        0b00001,
        0b11110,
        0b00000,
    ]),
    ('Y', [
        0b10001,
        0b10001,
        0b01010,
        0b00100,
        0b00100,
        0b00100,
        0b00000,
    ]),
    ('N', [
        0b10001,
        0b11001,
        0b10101,
        0b10011,
        0b10001,
        0b10001,
        0b00000,
    ]),
    ('T', [
        0b11111,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b00000,
    ]),
    ('A', [
        0b01110,
        0b10001,
        0b10001,
        0b11111,
        0b10001,
        0b10001,
        0b00000,
    ]),
    ('X', [
        0b10001,
        0b01010,
        0b00100,
        0b01010,
        0b10001,
        0b10001,
        0b00000,
    ]),
    ('Q', [
        0b01110,
        0b10001,
        0b10001,
        0b10101,
        0b01110,
        0b00011,
        0b00000,
    ]),
    ('C', [
        0b01110,
        0b10001,
        0b10000,
        0b10000,
        0b10001,
        0b01110,
        0b00000,
    ]),
    ('L', [
        0b10000,
        0b10000,
        0b10000,
        0b10000,
        0b10000,
        0b11111,
        0b00000,
    ]),
    ('O', [
        0b01110,
        0b10001,
        0b10001,
        0b10001,
        0b10001,
        0b01110,
        0b00000,
    ]),
    ('P', [
        0b11110,
        0b10001,
        0b11110,
        0b10000,
        0b10000,
        0b10000,
        0b00000,
    ]),
    ('M', [
        0b10001,
        0b11011,
        0b10101,
        0b10001,
        0b10001,
        0b10001,
        0b00000,
    ]),
    (' ', [
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
    ]),
];

pub fn get_glyph_bitmap(c: char) -> [u8; 7] {
    let upper = c.to_ascii_uppercase();
    for &(ch, rows) in FONT_5X7 {
        if ch == upper || ch == c {
            return rows;
        }
    }
    // Unknown character: blank
    [0; 7]
}

#[derive(Debug, Clone, Copy)]
pub struct DotMatrixConfig {
    pub dot_radius: i32,
    pub dot_spacing: i32,
    pub char_spacing: i32,
    pub show_unlit_dots: bool,
}

impl Default for DotMatrixConfig {
    fn default() -> Self {
        Self {
            dot_radius: 2,
            dot_spacing: 6,
            char_spacing: 6,
            show_unlit_dots: true,
        }
    }
}

impl DotMatrixConfig {
    pub fn char_width(&self) -> i32 {
        5 * self.dot_spacing
    }

    pub fn char_height(&self) -> i32 {
        7 * self.dot_spacing
    }

    pub fn text_width(&self, char_count: usize) -> i32 {
        if char_count == 0 {
            0
        } else {
            char_count as i32 * (self.char_width() + self.char_spacing) - self.char_spacing
        }
    }
}

/// Render a single character cell as dot-matrix pixels
pub fn render_char_dot_matrix(
    buffer: &mut [u32],
    buf_w: i32,
    buf_h: i32,
    start_x: i32,
    start_y: i32,
    c: char,
    config: &DotMatrixConfig,
    lit_color: Color,
    unlit_color: Color,
) {
    let bitmap = get_glyph_bitmap(c);

    for row in 0..7 {
        let row_bits = bitmap[row];
        for col in 0..5 {
            let is_lit = (row_bits & (1 << (4 - col))) != 0;
            let center_x = start_x + (col as i32) * config.dot_spacing + config.dot_radius;
            let center_y = start_y + (row as i32) * config.dot_spacing + config.dot_radius;

            if is_lit {
                // Subtle outer glow halo for lit dot
                let glow_col = lit_color.with_alpha((lit_color.a as u32 * 35 / 255) as u8);
                draw_dot(buffer, buf_w, buf_h, center_x, center_y, config.dot_radius + 1, glow_col);

                // Core active bright dot
                draw_dot(buffer, buf_w, buf_h, center_x, center_y, config.dot_radius, lit_color);
            } else if config.show_unlit_dots {
                // Faint unlit physical LCD crystal dot
                draw_dot(buffer, buf_w, buf_h, center_x, center_y, config.dot_radius, unlit_color);
            }
        }
    }
}

/// Render a string of text in dot matrix style.
/// Can be right-aligned (for calculator numbers) or left-aligned.
pub fn render_string_dot_matrix(
    buffer: &mut [u32],
    buf_w: i32,
    buf_h: i32,
    x: i32,
    y: i32,
    text: &str,
    max_slots: usize,
    right_aligned: bool,
    config: &DotMatrixConfig,
    lit_color: Color,
    unlit_color: Color,
) {
    let chars: Vec<char> = text.chars().collect();
    let count = chars.len();

    // In a physical LCD, there is a fixed number of digit slots (e.g. 10 or 12 slots)
    let display_slots = max_slots.max(count);
    let total_width = config.text_width(display_slots);

    let start_x = if right_aligned {
        x - total_width
    } else {
        x
    };

    let step = config.char_width() + config.char_spacing;

    for slot in 0..display_slots {
        let slot_x = start_x + slot as i32 * step;

        let char_to_draw = if right_aligned {
            let offset_from_right = display_slots - 1 - slot;
            if offset_from_right < count {
                chars[count - 1 - offset_from_right]
            } else {
                ' ' // empty slot
            }
        } else {
            if slot < count {
                chars[slot]
            } else {
                ' '
            }
        };

        render_char_dot_matrix(
            buffer,
            buf_w,
            buf_h,
            slot_x,
            y,
            char_to_draw,
            config,
            lit_color,
            unlit_color,
        );
    }
}

/// Draw a round dot at (cx, cy) with radius r and alpha blending
#[inline]
pub fn draw_dot(
    buffer: &mut [u32],
    buf_w: i32,
    buf_h: i32,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
) {
    if color.a == 0 {
        return;
    }

    let r_sq = radius * radius;
    let min_x = (cx - radius).max(0);
    let max_x = (cx + radius).min(buf_w - 1);
    let min_y = (cy - radius).max(0);
    let max_y = (cy + radius).min(buf_h - 1);

    for y in min_y..=max_y {
        let dy = y - cy;
        let dy_sq = dy * dy;
        let row_idx = y as usize * buf_w as usize;

        for x in min_x..=max_x {
            let dx = x - cx;
            let dist_sq = dx * dx + dy_sq;

            if dist_sq <= r_sq {
                let pixel_idx = row_idx + x as usize;
                let dst_u32 = buffer[pixel_idx];
                let dst_col = Color::from_rgba(
                    ((dst_u32 >> 16) & 0xFF) as u8,
                    ((dst_u32 >> 8) & 0xFF) as u8,
                    (dst_u32 & 0xFF) as u8,
                    ((dst_u32 >> 24) & 0xFF) as u8,
                );

                let blended = Color::blend_over(color, dst_col);
                buffer[pixel_idx] = blended.to_u32();
            }
        }
    }
}
