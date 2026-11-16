use crate::calculator::Calculator;
use crate::dotmatrix::{render_string_dot_matrix, DotMatrixConfig};
use crate::theme::{Color, Palette};
use crate::ui::{Button, ButtonKind, CalculatorLayout};
use fontdue::{Font, FontSettings};
use std::fs;
use std::path::Path;

pub struct Renderer {
    pub font: Option<Font>,
}

impl Renderer {
    pub fn new() -> Self {
        let font_candidates = [
            "/usr/share/fonts/TTF/DejaVuSansMono-Bold.ttf",
            "/usr/share/fonts/TTF/MesloLGLNerdFontMono-Bold.ttf",
            "/usr/share/fonts/TTF/FantasqueSansMNerdFontMono-Bold.ttf",
            "/usr/share/fonts/Adwaita/AdwaitaMono-Bold.ttf",
            "/usr/share/fonts/TTF/DejaVuSansMono.ttf",
        ];

        let mut font = None;
        for path_str in font_candidates {
            if Path::new(path_str).exists() {
                if let Ok(data) = fs::read(path_str) {
                    if let Ok(f) = Font::from_bytes(data, FontSettings::default()) {
                        font = Some(f);
                        break;
                    }
                }
            }
        }

        Self { font }
    }

    pub fn render(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        layout: &CalculatorLayout,
        calc: &Calculator,
        palette: &Palette,
        hovered_btn: Option<&str>,
        pressed_btn: Option<&str>,
    ) {
        // 1. Clear background with window surface color (preserving alpha for Wayland compositor)
        for pixel in buffer.iter_mut() {
            *pixel = palette.background.to_u32();
        }

        // 2. Draw btop++ style main window border
        draw_rounded_rect(
            buffer,
            width,
            height,
            2,
            2,
            width - 4,
            height - 4,
            12,
            Color::from_rgba(0, 0, 0, 0), // Transparent inside
            palette.outline.with_alpha(200),
            1,
        );

        // 3. Render Header (btop++ style terminal header)
        self.render_header(buffer, width, height, layout, palette);

        // 4. Render LCD Screen (Retro Dot-Matrix Display)
        self.render_lcd_screen(buffer, width, height, layout, calc, palette);

        // 5. Render Keypad Buttons
        self.render_keypad(buffer, width, height, layout, palette, hovered_btn, pressed_btn);

        // 6. Render Footer Hints
        self.render_footer(buffer, width, height, layout, palette);
    }

    fn render_header(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        layout: &CalculatorLayout,
        palette: &Palette,
    ) {
        let r = &layout.header_rect;

        // Header box background
        draw_rounded_rect(
            buffer,
            width,
            height,
            r.x,
            r.y,
            r.w,
            r.h,
            6,
            palette.surface_variant.with_alpha(120),
            palette.outline_variant.with_alpha(150),
            1,
        );

        if let Some(ref font) = self.font {
            // Title
            let safe_title = "CALCULADORA // AGY";
            self.draw_text(buffer, width, height, font, safe_title, r.x + 12, r.y + 8, 14.0, palette.primary);

            // Right status badge (e.g. Noctalia v5 indicator)
            let badge = if palette.name.starts_with("Noctalia Dynamic") {
                "NOCTALIA v5 [DYN]"
            } else {
                "NOCTALIA v5 [DARK]"
            };
            let (tw, _) = self.measure_text(font, badge, 11.0);
            let bx = r.x + r.w - tw - 12;
            self.draw_text(buffer, width, height, font, badge, bx, r.y + 10, 11.0, palette.text_muted);
        }
    }

    fn render_lcd_screen(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        layout: &CalculatorLayout,
        calc: &Calculator,
        palette: &Palette,
    ) {
        let r = &layout.screen_rect;

        // Outer LCD bevel / frame
        draw_rounded_rect(
            buffer,
            width,
            height,
            r.x,
            r.y,
            r.w,
            r.h,
            8,
            palette.surface_lowest,
            palette.outline.with_alpha(240),
            2,
        );

        // Inner recessed screen glass tint
        draw_rounded_rect(
            buffer,
            width,
            height,
            r.x + 3,
            r.y + 3,
            r.w - 6,
            r.h - 6,
            6,
            palette.surface_lowest.with_alpha(245),
            Color::from_rgba(0, 0, 0, 100),
            1,
        );

        // Top Status row inside LCD: [DEG] [NIRI] [MEM]
        if let Some(ref font) = self.font {
            let mode_str = "DEG";
            self.draw_text(buffer, width, height, font, mode_str, r.x + 10, r.y + 8, 10.0, palette.text_muted.with_alpha(150));

            let sys_str = "WAYLAND • NIRI";
            let (sw, _) = self.measure_text(font, sys_str, 10.0);
            self.draw_text(buffer, width, height, font, sys_str, r.x + r.w - sw - 10, r.y + 8, 10.0, palette.text_muted.with_alpha(130));
        }

        // History / Expression row (top line inside LCD)
        // Rendered with smaller dot matrix or clean monospace text
        let history_str = calc.history_text();
        let hist_cfg = DotMatrixConfig {
            dot_radius: 1,
            dot_spacing: 3,
            char_spacing: 3,
            show_unlit_dots: false,
        };

        let hist_y = r.y + 26;
        render_string_dot_matrix(
            buffer,
            width,
            height,
            r.x + r.w - 12,
            hist_y,
            &history_str,
            24,
            true, // right-aligned
            &hist_cfg,
            palette.text_muted.with_alpha(200),
            palette.outline_variant.with_alpha(15),
        );

        // Decorative subtle horizontal divider line inside LCD
        draw_line(
            buffer,
            width,
            height,
            r.x + 8,
            r.y + 45,
            r.x + r.w - 8,
            r.y + 45,
            palette.outline.with_alpha(80),
        );

        // Main Display row (large retro dot-matrix LCD font)
        // With active lit glowing dots and faint unlit crystal grid!
        let main_cfg = DotMatrixConfig {
            dot_radius: 2,
            dot_spacing: 6,
            char_spacing: 7,
            show_unlit_dots: true,
        };

        let display_text = calc.display_text();
        let main_y = r.y + 54;
        let lit_color = if calc.error.is_some() {
            palette.error
        } else {
            palette.primary
        };
        let unlit_color = palette.primary.with_alpha(18); // Faint unlit physical LCD crystal dot

        // Max slots that fit in screen width
        let slot_w = main_cfg.char_width() + main_cfg.char_spacing;
        let available_w = r.w - 24;
        let max_slots = (available_w / slot_w).max(8) as usize;

        render_string_dot_matrix(
            buffer,
            width,
            height,
            r.x + r.w - 12,
            main_y,
            &display_text,
            max_slots,
            true, // right-aligned
            &main_cfg,
            lit_color,
            unlit_color,
        );
    }

    fn render_keypad(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        layout: &CalculatorLayout,
        palette: &Palette,
        hovered_btn: Option<&str>,
        pressed_btn: Option<&str>,
    ) {
        for btn in &layout.buttons {
            self.render_button(buffer, width, height, btn, palette, hovered_btn, pressed_btn);
        }
    }

    fn render_button(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        btn: &Button,
        palette: &Palette,
        hovered_btn: Option<&str>,
        pressed_btn: Option<&str>,
    ) {
        let is_hovered = hovered_btn == Some(btn.id);
        let is_pressed = pressed_btn == Some(btn.id);
        let r = &btn.rect;

        // Button colors based on kind
        let (base_bg, border_col, text_col) = match btn.kind {
            ButtonKind::Digit => (
                palette.surface_variant.with_alpha(150),
                palette.outline_variant.with_alpha(180),
                palette.text,
            ),
            ButtonKind::Operator => (
                palette.secondary.with_alpha(35),
                palette.secondary.with_alpha(130),
                palette.secondary,
            ),
            ButtonKind::Function => (
                palette.tertiary.with_alpha(35),
                palette.tertiary.with_alpha(130),
                palette.tertiary,
            ),
            ButtonKind::Action => (
                palette.error.with_alpha(35),
                palette.error.with_alpha(130),
                palette.error,
            ),
            ButtonKind::Equals => (
                palette.primary.with_alpha(190),
                palette.primary,
                palette.on_primary,
            ),
        };

        let (final_bg, final_border, final_text) = if is_pressed {
            (
                palette.active,
                palette.primary,
                palette.on_primary,
            )
        } else if is_hovered {
            (
                Color::blend_over(palette.hover, base_bg),
                palette.hover.with_alpha(240),
                if btn.kind == ButtonKind::Equals { palette.on_primary } else { palette.text },
            )
        } else {
            (base_bg, border_col, text_col)
        };

        // Draw rounded button box
        draw_rounded_rect(
            buffer,
            width,
            height,
            r.x,
            r.y,
            r.w,
            r.h,
            8,
            final_bg,
            final_border,
            if is_hovered || is_pressed { 2 } else { 1 },
        );

        // Draw button label
        if let Some(ref font) = self.font {
            let font_size = if btn.label.len() > 2 { 14.0 } else { 18.0 };
            let (tw, th) = self.measure_text(font, btn.label, font_size);
            let tx = r.x + (r.w - tw) / 2;
            let ty = r.y + (r.h - th) / 2;

            self.draw_text(buffer, width, height, font, btn.label, tx, ty, font_size, final_text);

            // Small shortcut hint in the corner
            if !btn.sublabel.is_empty() && r.h > 40 && r.w > 40 {
                let hint_col = palette.text_muted.with_alpha(100);
                let (sw, _) = self.measure_text(font, btn.sublabel, 9.0);
                let sx = r.x + r.w - sw - 5;
                let sy = r.y + r.h - 13;
                self.draw_text(buffer, width, height, font, btn.sublabel, sx, sy, 9.0, hint_col);
            }
        }
    }

    fn render_footer(
        &self,
        buffer: &mut [u32],
        width: i32,
        height: i32,
        layout: &CalculatorLayout,
        palette: &Palette,
    ) {
        if let Some(ref font) = self.font {
            let hints = "[Esc] Salir • [Enter] = • [c] Borrar • [r] √ • [^] Pot • [%] Porc";
            let (tw, _) = self.measure_text(font, hints, 10.0);
            let fx = (width - tw) / 2;
            let fy = layout.footer_rect.y + 4;
            self.draw_text(buffer, width, height, font, hints, fx, fy, 10.0, palette.text_muted.with_alpha(160));
        }
    }

    pub fn draw_text(
        &self,
        buffer: &mut [u32],
        buf_w: i32,
        buf_h: i32,
        font: &Font,
        text: &str,
        start_x: i32,
        start_y: i32,
        size_px: f32,
        color: Color,
    ) {
        let mut cur_x = start_x;
        for c in text.chars() {
            let (metrics, bitmap) = font.rasterize(c, size_px);

            let glyph_x = cur_x + metrics.xmin;
            let glyph_y = start_y + (size_px as i32 - metrics.height as i32 - metrics.ymin);

            for row in 0..metrics.height {
                let y = glyph_y + row as i32;
                if y < 0 || y >= buf_h {
                    continue;
                }
                let row_buf_idx = y as usize * buf_w as usize;
                let row_bmp_idx = row * metrics.width;

                for col in 0..metrics.width {
                    let x = glyph_x + col as i32;
                    if x < 0 || x >= buf_w {
                        continue;
                    }

                    let alpha_coverage = bitmap[row_bmp_idx + col];
                    if alpha_coverage == 0 {
                        continue;
                    }

                    let text_pixel_alpha = (color.a as u32 * alpha_coverage as u32 / 255) as u8;
                    let text_pixel_col = color.with_alpha(text_pixel_alpha);

                    let buf_idx = row_buf_idx + x as usize;
                    let dst_u32 = buffer[buf_idx];
                    let dst_col = Color::from_rgba(
                        ((dst_u32 >> 16) & 0xFF) as u8,
                        ((dst_u32 >> 8) & 0xFF) as u8,
                        (dst_u32 & 0xFF) as u8,
                        ((dst_u32 >> 24) & 0xFF) as u8,
                    );

                    buffer[buf_idx] = Color::blend_over(text_pixel_col, dst_col).to_u32();
                }
            }

            cur_x += metrics.advance_width.round() as i32;
        }
    }

    pub fn measure_text(&self, font: &Font, text: &str, size_px: f32) -> (i32, i32) {
        let mut width = 0.0;
        let mut height = size_px;
        for c in text.chars() {
            let metrics = font.metrics(c, size_px);
            width += metrics.advance_width;
            if metrics.height as f32 > height {
                height = metrics.height as f32;
            }
        }
        (width.round() as i32, height.round() as i32)
    }
}

pub fn draw_line(
    buffer: &mut [u32],
    buf_w: i32,
    buf_h: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    if y0 == y1 {
        // Fast horizontal line
        let y = y0;
        if y < 0 || y >= buf_h {
            return;
        }
        let min_x = x0.min(x1).max(0);
        let max_x = x0.max(x1).min(buf_w - 1);
        let row_idx = y as usize * buf_w as usize;
        for x in min_x..=max_x {
            let idx = row_idx + x as usize;
            let dst_u32 = buffer[idx];
            let dst_col = Color::from_rgba(
                ((dst_u32 >> 16) & 0xFF) as u8,
                ((dst_u32 >> 8) & 0xFF) as u8,
                (dst_u32 & 0xFF) as u8,
                ((dst_u32 >> 24) & 0xFF) as u8,
            );
            buffer[idx] = Color::blend_over(color, dst_col).to_u32();
        }
    }
}

pub fn draw_rounded_rect(
    buffer: &mut [u32],
    buf_w: i32,
    buf_h: i32,
    rx: i32,
    ry: i32,
    rw: i32,
    rh: i32,
    radius: i32,
    fill_col: Color,
    border_col: Color,
    border_thick: i32,
) {
    let min_x = rx.max(0);
    let max_x = (rx + rw - 1).min(buf_w - 1);
    let min_y = ry.max(0);
    let max_y = (ry + rh - 1).min(buf_h - 1);

    let rad = radius.min(rw / 2).min(rh / 2);
    let rad_sq = rad * rad;
    let inner_rad = (rad - border_thick).max(0);
    let inner_rad_sq = inner_rad * inner_rad;

    for y in min_y..=max_y {
        let row_idx = y as usize * buf_w as usize;
        for x in min_x..=max_x {
            // Determine corner distance
            let cx = if x < rx + rad {
                rx + rad
            } else if x >= rx + rw - rad {
                rx + rw - 1 - rad
            } else {
                x
            };

            let cy = if y < ry + rad {
                ry + rad
            } else if y >= ry + rh - rad {
                ry + rh - 1 - rad
            } else {
                y
            };

            let dx = x - cx;
            let dy = y - cy;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq > rad_sq {
                // Outside rounded corner
                continue;
            }

            // Check if on border
            let is_border = if dist_sq > inner_rad_sq {
                true
            } else {
                x < rx + border_thick
                    || x >= rx + rw - border_thick
                    || y < ry + border_thick
                    || y >= ry + rh - border_thick
            };

            let color_to_draw = if is_border {
                border_col
            } else {
                fill_col
            };

            if color_to_draw.a == 0 {
                continue;
            }

            let idx = row_idx + x as usize;
            let dst_u32 = buffer[idx];
            let dst_col = Color::from_rgba(
                ((dst_u32 >> 16) & 0xFF) as u8,
                ((dst_u32 >> 8) & 0xFF) as u8,
                (dst_u32 & 0xFF) as u8,
                ((dst_u32 >> 24) & 0xFF) as u8,
            );

            buffer[idx] = Color::blend_over(color_to_draw, dst_col).to_u32();
        }
    }
}
