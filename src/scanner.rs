use std::fmt::Display;

#[derive(Debug)]
pub enum Token {
    S(String),
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S(v) => write!(f, "{}", v),
        }
    }
}
#[derive(Debug)]
pub struct Scanner {
    source: String,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        self.source
            .split(' ')
            .map(|s| Token::S(s.to_string()))
            .collect()
    }
}
