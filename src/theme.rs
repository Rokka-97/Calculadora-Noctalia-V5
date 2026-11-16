use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn with_alpha(self, a: u8) -> Self {
        Self { a, ..self }
    }

    /// Parse hex string like "#RRGGBB" or "RRGGBB" or with alpha "#AARRGGBB" / "#RRGGBBAA"
    pub fn parse_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim().trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::from_rgb(r, g, b))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Self::from_rgba(r, g, b, a))
        } else {
            None
        }
    }

    /// Convert to ARGB 32-bit integer for softbuffer framebuffer
    #[inline(always)]
    pub fn to_u32(self) -> u32 {
        ((self.a as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.g as u32) << 8)
            | (self.b as u32)
    }

    /// Alpha blend source color over destination color
    pub fn blend_over(src: Color, dst: Color) -> Color {
        let sa = src.a as u32;
        let da = dst.a as u32;
        let out_a = sa + da * (255 - sa) / 255;
        if out_a == 0 {
            return Color::from_rgba(0, 0, 0, 0);
        }

        let r = (src.r as u32 * sa + dst.r as u32 * da * (255 - sa) / 255) / out_a;
        let g = (src.g as u32 * sa + dst.g as u32 * da * (255 - sa) / 255) / out_a;
        let b = (src.b as u32 * sa + dst.b as u32 * da * (255 - sa) / 255) / out_a;

        Color::from_rgba(r as u8, g as u8, b as u8, out_a as u8)
    }
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub background: Color,       // Window background with alpha
    pub surface: Color,          // Container surfaces
    pub surface_variant: Color,  // Key buttons, inner frames
    pub surface_lowest: Color,   // LCD screen backing glass
    pub primary: Color,          // LCD active dot glow, equals button
    pub on_primary: Color,       // Text on primary
    pub secondary: Color,        // Operator buttons (+, -, *, /)
    pub tertiary: Color,         // Function buttons (^, √, %)
    pub outline: Color,          // btop++ borders & division lines
    pub outline_variant: Color,  // Subtle borders
    pub error: Color,            // Clear (C, AC), error states
    pub text: Color,             // Text labels
    pub text_muted: Color,       // Subdued status labels
    pub hover: Color,            // Button hover highlight
    pub active: Color,           // Button pressed highlight
}

impl Default for Palette {
    fn default() -> Self {
        // Built-in Noctalia Dark default scheme
        Self {
            name: "Noctalia Default (Dark)".to_string(),
            background: Color::from_rgba(7, 7, 34, 210), // Translucent ~82%
            surface: Color::from_rgb(7, 7, 34),
            surface_variant: Color::from_rgba(17, 17, 45, 230),
            surface_lowest: Color::from_rgba(4, 4, 20, 240),
            primary: Color::from_rgb(255, 245, 155),    // #fff59b warm glow
            on_primary: Color::from_rgb(14, 14, 67),
            secondary: Color::from_rgb(169, 174, 254),  // #a9aefe lavender
            tertiary: Color::from_rgb(155, 254, 206),   // #9bfece mint green
            outline: Color::from_rgb(33, 33, 95),       // #21215f
            outline_variant: Color::from_rgb(20, 20, 60),
            error: Color::from_rgb(253, 70, 99),        // #fd4663 coral red
            text: Color::from_rgb(243, 237, 247),       // #f3edf7
            text_muted: Color::from_rgb(124, 128, 180), // #7c80b4
            hover: Color::from_rgba(155, 254, 206, 100),
            active: Color::from_rgba(255, 245, 155, 180),
        }
    }
}

pub struct ThemeManager {
    pub current: Arc<RwLock<Palette>>,
    pub updated: Arc<AtomicBool>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let palette = Self::load_noctalia_palette().unwrap_or_default();
        let current = Arc::new(RwLock::new(palette));
        let updated = Arc::new(AtomicBool::new(false));

        // Start watcher thread for automatic color updates on wallpaper / theme changes
        let cur_clone = current.clone();
        let up_clone = updated.clone();
        thread::spawn(move || {
            let mut last_mtime = None;
            let settings_path = get_home_dir()
                .map(|h| h.join(".local/state/noctalia/settings.toml"));

            loop {
                thread::sleep(Duration::from_millis(1500));

                if let Some(ref path) = settings_path {
                    if let Ok(metadata) = fs::metadata(path) {
                        if let Ok(mtime) = metadata.modified() {
                            if last_mtime.map_or(false, |last| mtime > last) {
                                // Settings modified! Reload palette
                                if let Ok(new_pal) = Self::load_noctalia_palette() {
                                    if let Ok(mut lock) = cur_clone.write() {
                                        *lock = new_pal;
                                        up_clone.store(true, Ordering::SeqCst);
                                    }
                                }
                            }
                            last_mtime = Some(mtime);
                        }
                    }
                }
            }
        });

        Self { current, updated }
    }

    /// Load the current active palette directly from Noctalia v5 configuration and CLI
    pub fn load_noctalia_palette() -> Result<Palette, String> {
        let home = get_home_dir().ok_or("Cannot find home directory")?;
        let settings_file = home.join(".local/state/noctalia/settings.toml");

        // Try reading settings.toml to get wallpaper path and scheme
        if settings_file.exists() {
            if let Ok(content) = fs::read_to_string(&settings_file) {
                if let Ok(settings) = toml::from_str::<toml::Value>(&content) {
                    let source = settings.get("theme")
                        .and_then(|t| t.get("source"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("wallpaper");

                    let scheme = settings.get("theme")
                        .and_then(|t| t.get("wallpaper_scheme"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("soft");

                    let mode = settings.get("theme")
                        .and_then(|t| t.get("mode"))
                        .and_then(|m| m.as_str())
                        .unwrap_or("dark");

                    if source == "wallpaper" {
                        // Get wallpaper path
                        let wp_path = settings.get("wallpaper")
                            .and_then(|w| w.get("default"))
                            .and_then(|d| d.get("path"))
                            .and_then(|p| p.as_str())
                            .or_else(|| {
                                settings.get("wallpaper")
                                    .and_then(|w| w.get("last"))
                                    .and_then(|d| d.get("path"))
                                    .and_then(|p| p.as_str())
                            });

                        if let Some(wp) = wp_path {
                            if Path::new(wp).exists() {
                                if let Ok(pal) = Self::generate_from_noctalia_cli(wp, scheme, mode) {
                                    return Ok(pal);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fallback: Check ~/.cache/noctalia/predefined-scheme.json
        let cache_scheme = home.join(".cache/noctalia/predefined-scheme.json");
        if cache_scheme.exists() {
            if let Ok(content) = fs::read_to_string(&cache_scheme) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(dark) = json.get("dark") {
                        return Self::parse_predefined_json(dark, "Noctalia Cache");
                    }
                }
            }
        }

        Ok(Palette::default())
    }

    /// Generate palette dynamically using `noctalia theme <image> --scheme <scheme> --dark`
    fn generate_from_noctalia_cli(wp_path: &str, scheme: &str, mode: &str) -> Result<Palette, String> {
        let mut cmd = Command::new("noctalia");
        cmd.arg("theme")
            .arg(wp_path)
            .arg("--scheme")
            .arg(scheme);

        if mode == "light" {
            cmd.arg("--light");
        } else {
            cmd.arg("--dark");
        }

        let output = cmd.output().map_err(|e| format!("Failed to run noctalia theme: {}", e))?;
        if !output.status.success() {
            return Err("noctalia theme command failed".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: HashMap<String, String> = serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse noctalia json: {}", e))?;

        let get_col = |key: &str| -> Option<Color> {
            json.get(key).and_then(|v| Color::parse_hex(v))
        };

        let surface = get_col("surface").unwrap_or(Color::from_rgb(18, 18, 28));
        let _surface_container = get_col("surface_container").unwrap_or(surface);
        let surface_variant = get_col("surface_variant").unwrap_or(surface);
        let lowest = get_col("surface_container_lowest")
            .unwrap_or(Color::from_rgb(10, 10, 18));
        let primary = get_col("primary").unwrap_or(Color::from_rgb(255, 245, 155));
        let on_primary = get_col("on_primary").unwrap_or(Color::from_rgb(20, 20, 40));
        let secondary = get_col("secondary").unwrap_or(Color::from_rgb(169, 174, 254));
        let tertiary = get_col("tertiary").unwrap_or(Color::from_rgb(155, 254, 206));
        let outline = get_col("outline").unwrap_or(Color::from_rgb(60, 60, 90));
        let outline_variant = get_col("outline_variant").unwrap_or(outline);
        let error = get_col("error").unwrap_or(Color::from_rgb(253, 70, 99));
        let on_surface = get_col("on_surface").unwrap_or(Color::from_rgb(240, 240, 245));
        let on_variant = get_col("on_surface_variant").unwrap_or(Color::from_rgb(150, 150, 170));
        let hover = get_col("hover").unwrap_or(tertiary);

        Ok(Palette {
            name: format!("Noctalia Dynamic ({} / {})", scheme, mode),
            background: surface.with_alpha(205), // ~80% transparency for Wayland compositor
            surface,
            surface_variant: surface_variant.with_alpha(230),
            surface_lowest: lowest.with_alpha(240),
            primary,
            on_primary,
            secondary,
            tertiary,
            outline,
            outline_variant,
            error,
            text: on_surface,
            text_muted: on_variant,
            hover: hover.with_alpha(85),
            active: primary.with_alpha(180),
        })
    }

    fn parse_predefined_json(json: &serde_json::Value, name: &str) -> Result<Palette, String> {
        let get = |k: &str| -> Option<Color> {
            json.get(k).and_then(|v| v.as_str()).and_then(Color::parse_hex)
        };

        let surface = get("mSurface").unwrap_or(Color::from_rgb(7, 7, 34));
        let primary = get("mPrimary").unwrap_or(Color::from_rgb(255, 245, 155));
        let on_primary = get("mOnPrimary").unwrap_or(Color::from_rgb(14, 14, 67));
        let secondary = get("mSecondary").unwrap_or(Color::from_rgb(169, 174, 254));
        let tertiary = get("mTertiary").unwrap_or(Color::from_rgb(155, 254, 206));
        let error = get("mError").unwrap_or(Color::from_rgb(253, 70, 99));
        let on_surface = get("mOnSurface").unwrap_or(Color::from_rgb(243, 237, 247));
        let on_variant = get("mOnSurfaceVariant").unwrap_or(Color::from_rgb(124, 128, 180));
        let outline = get("mOutline").unwrap_or(Color::from_rgb(33, 33, 95));
        let surface_var = get("mSurfaceVariant").unwrap_or(Color::from_rgb(17, 17, 45));

        Ok(Palette {
            name: name.to_string(),
            background: surface.with_alpha(210),
            surface,
            surface_variant: surface_var.with_alpha(230),
            surface_lowest: Color::from_rgba(4, 4, 20, 240),
            primary,
            on_primary,
            secondary,
            tertiary,
            outline,
            outline_variant: outline,
            error,
            text: on_surface,
            text_muted: on_variant,
            hover: tertiary.with_alpha(90),
            active: primary.with_alpha(180),
        })
    }
}

fn get_home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
