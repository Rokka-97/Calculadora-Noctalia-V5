/// Math engine for the calculator supporting PEMDAS, parentheses,
/// square roots, powers, percentages, and floating-point arithmetic.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Sqrt,
    Percent,
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    NegativeSquareRoot,
    SyntaxError,
    InvalidCharacter(char),
    EmptyExpression,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "ERR: DIV/0"),
            CalcError::NegativeSquareRoot => write!(f, "ERR: NEG SQRT"),
            CalcError::SyntaxError => write!(f, "ERR: SYNTAX"),
            CalcError::InvalidCharacter(c) => write!(f, "ERR: CHAR '{}'", c),
            CalcError::EmptyExpression => write!(f, "0"),
        }
    }
}

pub struct Calculator {
    /// Active raw expression input (e.g. "12 + 5 * √(4)")
    pub expression: String,
    /// Previous evaluated expression (e.g. "12 + 5 * √(4) =")
    pub history: String,
    /// Last calculated result
    pub last_result: Option<f64>,
    /// Last error encountered if any
    pub error: Option<CalcError>,
    /// Whether the next digit input should clear/replace the display
    pub replace_on_input: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            expression: String::new(),
            history: String::new(),
            last_result: None,
            error: None,
            replace_on_input: false,
        }
    }

    /// Push a character or token string into the active expression
    pub fn input_str(&mut self, text: &str) {
        if self.error.is_some() {
            self.error = None;
            self.expression.clear();
        }

        if self.replace_on_input {
            // If typing a digit after a result, start fresh
            if text.chars().all(|c| c.is_ascii_digit() || c == '.') {
                self.expression.clear();
            }
            self.replace_on_input = false;
        }

        self.expression.push_str(text);
    }

    /// Backspace: delete last character or clear error
    pub fn backspace(&mut self) {
        if self.error.is_some() {
            self.error = None;
            return;
        }
        if self.replace_on_input {
            self.replace_on_input = false;
        }
        // Handle multi-byte unicode like '√' properly
        self.expression.pop();
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.expression.clear();
        self.error = None;
        self.replace_on_input = false;
    }

    /// Clear everything including history
    pub fn clear_all(&mut self) {
        self.clear();
        self.history.clear();
        self.last_result = None;
    }

    /// Toggle sign of the last active number or enclose in -()
    pub fn toggle_sign(&mut self) {
        if self.expression.is_empty() {
            if let Some(res) = self.last_result {
                let neg = -res;
                self.expression = format_number(neg);
                self.last_result = Some(neg);
            }
            return;
        }

        // Check if expression is just a number
        if let Ok(num) = self.expression.parse::<f64>() {
            self.expression = format_number(-num);
        } else {
            // Wrap in - ( ... )
            self.expression = format!("-({})", self.expression);
        }
    }

    /// Evaluate current expression and update state
    pub fn evaluate(&mut self) -> Result<f64, CalcError> {
        if self.expression.trim().is_empty() {
            if let Some(res) = self.last_result {
                return Ok(res);
            }
            return Ok(0.0);
        }

        let clean_expr = self.expression.clone();
        match evaluate_expression(&clean_expr) {
            Ok(res) => {
                self.history = format!("{} =", clean_expr);
                self.last_result = Some(res);
                self.expression = format_number(res);
                self.error = None;
                self.replace_on_input = true;
                Ok(res)
            }
            Err(e) => {
                self.error = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Current display string to show on the main LCD row
    pub fn display_text(&self) -> String {
        if let Some(ref err) = self.error {
            err.to_string()
        } else if self.expression.is_empty() {
            if let Some(res) = self.last_result {
                format_number(res)
            } else {
                "0".to_string()
            }
        } else {
            self.expression.clone()
        }
    }

    /// Top LCD row showing previous expression / history
    pub fn history_text(&self) -> String {
        if !self.history.is_empty() {
            self.history.clone()
        } else {
            "CALC READY".to_string()
        }
    }
}

pub fn format_number(val: f64) -> String {
    if val.is_nan() {
        return "NAN".to_string();
    }
    if val.is_infinite() {
        return if val > 0.0 { "INF".to_string() } else { "-INF".to_string() };
    }

    // Format integer if exactly or extremely close to integer
    if (val - val.round()).abs() < 1e-11 && val.abs() < 1e14 {
        format!("{}", val.round() as i64)
    } else {
        // Format with up to 8 decimal places and strip trailing zeros
        let s = format!("{:.8}", val);
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    }
}

/// Tokenizer for mathematical expressions
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                let mut has_dot = c == '.';
                i += 1;
                while i < chars.len() {
                    let next = chars[i];
                    if next.is_ascii_digit() {
                        i += 1;
                    } else if next == '.' {
                        if has_dot {
                            return Err(CalcError::SyntaxError);
                        }
                        has_dot = true;
                        i += 1;
                    } else {
                        break;
                    }
                }
                let num_str: String = chars[start..i].iter().collect();
                let num = num_str.parse::<f64>().map_err(|_| CalcError::SyntaxError)?;
                tokens.push(Token::Number(num));
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' | 'x' | 'X' | '×' => {
                tokens.push(Token::Multiply);
                i += 1;
            }
            '/' | '÷' => {
                tokens.push(Token::Divide);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Power);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
            }
            '√' => {
                tokens.push(Token::Sqrt);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            // Check for "sqrt" written out
            's' | 'S' => {
                if i + 3 < chars.len()
                    && (chars[i..i + 4].iter().collect::<String>().to_lowercase() == "sqrt")
                {
                    tokens.push(Token::Sqrt);
                    i += 4;
                } else {
                    return Err(CalcError::InvalidCharacter(c));
                }
            }
            _ => {
                return Err(CalcError::InvalidCharacter(c));
            }
        }
    }

    Ok(tokens)
}

/// Recursive descent parser for expressions:
/// Grammar:
/// Expression -> Term (('+' | '-') Term)*
/// Term       -> Factor (('*' | '/') Factor)*
/// Factor     -> Power ('%' Factor)*
/// Power      -> Unary ('^' Power)?
/// Unary      -> ('+' | '-')* Unary | '√' Unary | Primary ('%')*
/// Primary    -> Number | '(' Expression ')'
pub fn evaluate_expression(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err(CalcError::EmptyExpression);
    }
    let mut pos = 0;
    let res = parse_expression(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(CalcError::SyntaxError);
    }
    Ok(res)
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Result<f64, CalcError> {
    let mut left = parse_term(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Plus => {
                *pos += 1;
                let right = parse_term(tokens, pos)?;
                // Handle percentage addition: e.g. 100 + 10% = 100 + (100 * 0.10)
                // Note: if right was parsed with a trailing %, check tokens
                left += right;
            }
            Token::Minus => {
                *pos += 1;
                let right = parse_term(tokens, pos)?;
                left -= right;
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Result<f64, CalcError> {
    let mut left = parse_power(tokens, pos)?;

    while *pos < tokens.len() {
        match tokens[*pos] {
            Token::Multiply => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                left *= right;
            }
            Token::Divide => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                if right == 0.0 {
                    return Err(CalcError::DivisionByZero);
                }
                left /= right;
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_power(tokens: &[Token], pos: &mut usize) -> Result<f64, CalcError> {
    let left = parse_unary(tokens, pos)?;

    if *pos < tokens.len() && tokens[*pos] == Token::Power {
        *pos += 1;
        let right = parse_power(tokens, pos)?;
        // Exponentiation
        let res = left.powf(right);
        if res.is_nan() {
            return Err(CalcError::SyntaxError);
        }
        Ok(res)
    } else {
        Ok(left)
    }
}

fn parse_unary(tokens: &[Token], pos: &mut usize) -> Result<f64, CalcError> {
    if *pos >= tokens.len() {
        return Err(CalcError::SyntaxError);
    }

    match tokens[*pos] {
        Token::Plus => {
            *pos += 1;
            parse_unary(tokens, pos)
        }
        Token::Minus => {
            *pos += 1;
            let val = parse_unary(tokens, pos)?;
            Ok(-val)
        }
        Token::Sqrt => {
            *pos += 1;
            let val = parse_unary(tokens, pos)?;
            if val < 0.0 {
                return Err(CalcError::NegativeSquareRoot);
            }
            Ok(val.sqrt())
        }
        _ => {
            let mut val = parse_primary(tokens, pos)?;
            // Check for trailing % (e.g. 50%)
            while *pos < tokens.len() && tokens[*pos] == Token::Percent {
                *pos += 1;
                val /= 100.0;
            }
            Ok(val)
        }
    }
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<f64, CalcError> {
    if *pos >= tokens.len() {
        return Err(CalcError::SyntaxError);
    }

    match &tokens[*pos] {
        Token::Number(n) => {
            let val = *n;
            *pos += 1;
            Ok(val)
        }
        Token::LParen => {
            *pos += 1;
            let val = parse_expression(tokens, pos)?;
            if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                return Err(CalcError::SyntaxError);
            }
            *pos += 1;
            Ok(val)
        }
        _ => Err(CalcError::SyntaxError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
        assert_eq!(evaluate_expression("(2 + 3) * 4").unwrap(), 20.0);
        assert_eq!(evaluate_expression("10 - 4 / 2").unwrap(), 8.0);
        assert_eq!(evaluate_expression("10 / 4").unwrap(), 2.5);
    }

    #[test]
    fn test_powers_and_roots() {
        assert_eq!(evaluate_expression("2 ^ 3").unwrap(), 8.0);
        assert_eq!(evaluate_expression("√16").unwrap(), 4.0);
        assert_eq!(evaluate_expression("√(9 + 16)").unwrap(), 5.0);
        assert_eq!(evaluate_expression("3 + √4 * 2 ^ 3").unwrap(), 19.0);
        assert_eq!(evaluate_expression("25 + 5 * √(16)").unwrap(), 45.0);
    }

    #[test]
    fn test_percentages() {
        assert_eq!(evaluate_expression("50%").unwrap(), 0.5);
        assert_eq!(evaluate_expression("200 * 15%").unwrap(), 30.0);
    }

    #[test]
    fn test_errors() {
        assert_eq!(evaluate_expression("5 / 0"), Err(CalcError::DivisionByZero));
        assert_eq!(evaluate_expression("√(-4)"), Err(CalcError::NegativeSquareRoot));
        assert!(evaluate_expression("2 + (3 *").is_err());
    }
}
