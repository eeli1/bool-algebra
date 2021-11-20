#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum Token {
    And,      // ∧
    Or,       // ∨
    Xor,      // ⊕
    Not,      // ¬
    Eq,       // ≡
    ImplicAB, // →
    ImplicBA, // ←
    Nand,     // ⊼ |
    Nor,      // ⊽ ↓

    One,  // 1
    Zero, // 0

    Open,  // (
    Close, // )

    Var(String), // a
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::And => write!(f, "∧"),
            Token::Or => write!(f, "∨"),
            Token::Xor => write!(f, "⊕"),
            Token::Not => write!(f, "!"),
            Token::ImplicAB => write!(f, "→"),
            Token::ImplicBA => write!(f, "←"),
            Token::Eq => write!(f, "≡"),
            Token::Nand => write!(f, "⊼"),
            Token::Nor => write!(f, "⊽"),

            Token::One => write!(f, "1"),
            Token::Zero => write!(f, "0"),

            Token::Open => write!(f, "("),
            Token::Close => write!(f, ")"),

            Token::Var(name) => write!(f, "{}", name),
        }
    }
}
