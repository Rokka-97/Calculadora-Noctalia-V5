#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonKind {
    Digit,
    Operator,
    Function,
    Action,
    Equals,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub id: &'static str,
    pub label: &'static str,
    pub sublabel: &'static str, // e.g. keyboard shortcut hint
    pub kind: ButtonKind,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }
}

pub struct CalculatorLayout {
    pub window_rect: Rect,
    pub header_rect: Rect,
    pub screen_rect: Rect,
    pub keypad_rect: Rect,
    pub footer_rect: Rect,
    pub buttons: Vec<Button>,
}

impl CalculatorLayout {
    pub fn calculate(width: i32, height: i32) -> Self {
        let pad = 14;
        let header_h = 32;
        let screen_h = 115;
        let footer_h = 24;

        let window_rect = Rect::new(0, 0, width, height);
        let header_rect = Rect::new(pad, pad, width - pad * 2, header_h);
        let screen_rect = Rect::new(pad, pad + header_h + 8, width - pad * 2, screen_h);

        let keypad_top = screen_rect.y + screen_rect.h + 12;
        let keypad_h = height - keypad_top - footer_h - pad - 6;
        let keypad_rect = Rect::new(pad, keypad_top, width - pad * 2, keypad_h.max(200));

        let footer_rect = Rect::new(pad, height - footer_h - 8, width - pad * 2, footer_h);

        // Build 5x5 grid of buttons
        let rows = 5;
        let cols = 5;
        let gap = 8;

        let btn_w = (keypad_rect.w - (cols - 1) * gap) / cols;
        let btn_h = (keypad_rect.h - (rows - 1) * gap) / rows;

        let grid_def: [[(&'static str, &'static str, &'static str, ButtonKind); 5]; 5] = [
            [
                ("C", "C", "c", ButtonKind::Action),
                ("(", "(", "(", ButtonKind::Function),
                (")", ")", ")", ButtonKind::Function),
                ("%", "%", "%", ButtonKind::Function),
                ("/", "÷", "/", ButtonKind::Operator),
            ],
            [
                ("7", "7", "7", ButtonKind::Digit),
                ("8", "8", "8", ButtonKind::Digit),
                ("9", "9", "9", ButtonKind::Digit),
                ("*", "×", "*", ButtonKind::Operator),
                ("sqrt", "√", "r", ButtonKind::Function),
            ],
            [
                ("4", "4", "4", ButtonKind::Digit),
                ("5", "5", "5", ButtonKind::Digit),
                ("6", "6", "6", ButtonKind::Digit),
                ("-", "−", "-", ButtonKind::Operator),
                ("^", "xʸ", "^", ButtonKind::Function),
            ],
            [
                ("1", "1", "1", ButtonKind::Digit),
                ("2", "2", "2", ButtonKind::Digit),
                ("3", "3", "3", ButtonKind::Digit),
                ("+", "+", "+", ButtonKind::Operator),
                ("+-", "±", "n", ButtonKind::Function),
            ],
            [
                ("0", "0", "0", ButtonKind::Digit),
                (".", "•", ".", ButtonKind::Digit),
                ("backspace", "DEL", "bs", ButtonKind::Action),
                ("=", "=", "ret", ButtonKind::Equals),
                ("AC", "AC", "esc", ButtonKind::Action),
            ],
        ];

        let mut buttons = Vec::with_capacity(rows as usize * cols as usize);

        for (r_idx, row) in grid_def.iter().enumerate() {
            let y = keypad_rect.y + r_idx as i32 * (btn_h + gap);
            for (c_idx, &(id, label, sublabel, kind)) in row.iter().enumerate() {
                let x = keypad_rect.x + c_idx as i32 * (btn_w + gap);
                buttons.push(Button {
                    id,
                    label,
                    sublabel,
                    kind,
                    rect: Rect::new(x, y, btn_w, btn_h),
                });
            }
        }

        Self {
            window_rect,
            header_rect,
            screen_rect,
            keypad_rect,
            footer_rect,
            buttons,
        }
    }

    pub fn find_button_at(&self, px: i32, py: i32) -> Option<&Button> {
        self.buttons.iter().find(|b| b.rect.contains(px, py))
    }
}
