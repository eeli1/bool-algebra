#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum Token {
    And, // &
    Or,  // |
    Xor, // ^
    Not, // !

    One,  // 1
    Zero, // 0

    Open,  // (
    Close, // )

    Var(String), // a
}