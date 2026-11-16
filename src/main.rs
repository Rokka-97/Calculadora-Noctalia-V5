pub mod calculator;
pub mod dotmatrix;
pub mod renderer;
pub mod theme;
pub mod ui;

use calculator::Calculator;
use renderer::Renderer;
use theme::ThemeManager;
use ui::CalculatorLayout;

use std::num::NonZeroU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{CursorIcon, Window, WindowId};

#[cfg(target_os = "linux")]
use winit::platform::wayland::WindowAttributesExtWayland;

struct App {
    theme_mgr: ThemeManager,
    renderer: Renderer,
    calc: Calculator,
    layout: CalculatorLayout,
    window: Option<Arc<Window>>,
    context: Option<softbuffer::Context<Arc<Window>>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    cursor_pos: (f64, f64),
    hovered_btn: Option<String>,
    pressed_btn: Option<String>,
    size: PhysicalSize<u32>,
}

impl App {
    fn new() -> Self {
        let theme_mgr = ThemeManager::new();
        let renderer = Renderer::new();
        let calc = Calculator::new();
        let initial_w = 390;
        let initial_h = 560;
        let layout = CalculatorLayout::calculate(initial_w, initial_h);

        Self {
            theme_mgr,
            renderer,
            calc,
            layout,
            window: None,
            context: None,
            surface: None,
            cursor_pos: (0.0, 0.0),
            hovered_btn: None,
            pressed_btn: None,
            size: PhysicalSize::new(initial_w as u32, initial_h as u32),
        }
    }

    fn handle_action(&mut self, id: &str) {
        match id {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                self.calc.input_str(id);
            }
            "+" => self.calc.input_str(" + "),
            "-" => self.calc.input_str(" - "),
            "*" => self.calc.input_str(" * "),
            "/" => self.calc.input_str(" / "),
            "^" => self.calc.input_str(" ^ "),
            "sqrt" => self.calc.input_str("√("),
            "%" => self.calc.input_str("%"),
            "(" => self.calc.input_str("("),
            ")" => self.calc.input_str(")"),
            "+-" => self.calc.toggle_sign(),
            "backspace" => self.calc.backspace(),
            "C" => self.calc.clear(),
            "AC" => self.calc.clear_all(),
            "=" => {
                let _ = self.calc.evaluate();
            }
            _ => {}
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let mut attrs = Window::default_attributes()
            .with_title("Calculadora AGY")
            .with_inner_size(LogicalSize::new(390.0, 560.0))
            .with_min_inner_size(LogicalSize::new(320.0, 460.0))
            .with_transparent(true);

        #[cfg(target_os = "linux")]
        {
            attrs = attrs.with_name("calculadora-agy", "calculadora-agy");
        }

        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                eprintln!("Error creating Wayland window: {}", e);
                return;
            }
        };

        let context = match softbuffer::Context::new(window.clone()) {
            Ok(ctx) => ctx,
            Err(e) => {
                eprintln!("Error creating softbuffer context: {}", e);
                return;
            }
        };

        let surface = match softbuffer::Surface::new(&context, window.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error creating softbuffer surface: {}", e);
                return;
            }
        };

        self.size = window.inner_size();
        self.layout = CalculatorLayout::calculate(self.size.width as i32, self.size.height as i32);
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    self.size = size;
                    self.layout = CalculatorLayout::calculate(size.width as i32, size.height as i32);
                    if let Some(ref window) = self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = (position.x, position.y);
                let px = position.x as i32;
                let py = position.y as i32;

                let is_header = self.layout.header_rect.contains(px, py);
                let new_hover = self
                    .layout
                    .find_button_at(px, py)
                    .map(|b| b.id.to_string());

                if let Some(ref window) = self.window {
                    if is_header {
                        window.set_cursor(CursorIcon::Grab);
                    } else if new_hover.is_some() {
                        window.set_cursor(CursorIcon::Pointer);
                    } else {
                        window.set_cursor(CursorIcon::Default);
                    }
                }

                if new_hover != self.hovered_btn {
                    self.hovered_btn = new_hover;
                    if let Some(ref window) = self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    match state {
                        ElementState::Pressed => {
                            let px = self.cursor_pos.0 as i32;
                            let py = self.cursor_pos.1 as i32;

                            // 1. If clicking header: drag window interactively with mouse
                            if self.layout.header_rect.contains(px, py) {
                                if let Some(ref window) = self.window {
                                    window.set_cursor(CursorIcon::Grabbing);
                                    let _ = window.drag_window();
                                }
                                return;
                            }

                            // 2. If clicking on button:
                            if let Some(btn) = self.layout.find_button_at(px, py) {
                                let id = btn.id.to_string();
                                self.pressed_btn = Some(id.clone());
                                self.handle_action(&id);
                                if let Some(ref window) = self.window {
                                    window.request_redraw();
                                }
                            }
                        }
                        ElementState::Released => {
                            if let Some(ref window) = self.window {
                                let px = self.cursor_pos.0 as i32;
                                let py = self.cursor_pos.1 as i32;
                                if self.layout.header_rect.contains(px, py) {
                                    window.set_cursor(CursorIcon::Grab);
                                }
                            }
                            if self.pressed_btn.is_some() {
                                self.pressed_btn = None;
                                if let Some(ref window) = self.window {
                                    window.request_redraw();
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key,
                        physical_key: _,
                        text,
                        ..
                    },
                ..
            } => {
                let mut handled = true;

                match logical_key {
                    Key::Named(NamedKey::Enter) => {
                        let _ = self.calc.evaluate();
                    }
                    Key::Named(NamedKey::Backspace) => {
                        self.calc.backspace();
                    }
                    Key::Named(NamedKey::Escape) => {
                        if self.calc.expression.is_empty() {
                            event_loop.exit();
                        } else {
                            self.calc.clear();
                        }
                    }
                    Key::Named(NamedKey::Delete) => {
                        self.calc.clear_all();
                    }
                    _ => {
                        if let Some(t) = text.as_ref().map(|s| s.as_str()) {
                            match t {
                                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." | "," => {
                                    let s = if t == "," { "." } else { t };
                                    self.calc.input_str(s);
                                }
                                "+" => self.calc.input_str(" + "),
                                "-" => self.calc.input_str(" - "),
                                "*" => self.calc.input_str(" * "),
                                "/" => self.calc.input_str(" / "),
                                "^" => self.calc.input_str(" ^ "),
                                "%" => self.calc.input_str("%"),
                                "(" => self.calc.input_str("("),
                                ")" => self.calc.input_str(")"),
                                "=" => {
                                    let _ = self.calc.evaluate();
                                }
                                "c" | "C" => {
                                    self.calc.clear();
                                }
                                "r" | "R" | "v" | "V" => {
                                    self.calc.input_str("√(");
                                }
                                "q" | "Q" => {
                                    if self.calc.expression.is_empty() {
                                        event_loop.exit();
                                    } else {
                                        self.calc.clear();
                                    }
                                }
                                _ => {
                                    handled = false;
                                }
                            }
                        } else {
                            handled = false;
                        }
                    }
                }

                if handled {
                    if let Some(ref window) = self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(surface), Some(_window)) = (&mut self.surface, &self.window) {
                    let w = self.size.width;
                    let h = self.size.height;

                    if w == 0 || h == 0 {
                        return;
                    }

                    if let (Some(w_nz), Some(h_nz)) = (NonZeroU32::new(w), NonZeroU32::new(h)) {
                        let _ = surface.resize(w_nz, h_nz);
                    }

                    if let Ok(mut buffer) = surface.buffer_mut() {
                        let palette = self.theme_mgr.current.read().unwrap().clone();
                        self.renderer.render(
                            &mut buffer,
                            w as i32,
                            h as i32,
                            &self.layout,
                            &self.calc,
                            &palette,
                            self.hovered_btn.as_deref(),
                            self.pressed_btn.as_deref(),
                        );
                        let _ = buffer.present();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // If theme watcher detected a wallpaper / theme update in Noctalia v5:
        if self.theme_mgr.updated.swap(false, Ordering::SeqCst) {
            if let Some(ref window) = self.window {
                window.request_redraw();
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);
}
