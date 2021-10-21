use bool_func_parser::Token;
const WITH_PARENTHESES: bool = true;

#[test]
fn dnf() {
    let table = vec![false, false, false, true];
    let names = vec!["a".to_string(), "b".to_string()];
    let dnf = if WITH_PARENTHESES {
        vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Var("b".to_string()),
            Token::Close,
        ]
    } else {
        vec![
            Token::Var("a".to_string()),
            Token::And,
            Token::Var("b".to_string()),
        ]
    };
    assert_eq!(bool_func_parser::dnf(&table, &names), Ok(dnf));

    let table = vec![true, true, false, false, true, true, false, false];
    let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let dnf = if WITH_PARENTHESES {
        vec![
            Token::Open,
            Token::Not,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Not,
            Token::Var("c".to_string()),
            Token::Close,
            Token::Or,
            Token::Open,
            Token::Not,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Var("c".to_string()),
            Token::Close,
            Token::Or,
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Not,
            Token::Var("c".to_string()),
            Token::Close,
            Token::Or,
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Var("c".to_string()),
            Token::Close,
        ]
    } else {
        vec![
            Token::Not,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Not,
            Token::Var("c".to_string()),
            Token::Or,
            Token::Not,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Var("c".to_string()),
            Token::Or,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Not,
            Token::Var("c".to_string()),
            Token::Or,
            Token::Var("a".to_string()),
            Token::And,
            Token::Not,
            Token::Var("b".to_string()),
            Token::And,
            Token::Var("c".to_string()),
        ]
    };
    assert_eq!(bool_func_parser::dnf(&table, &names), Ok(dnf));
}

#[test]
fn print_tabel() {
    let table = vec![false, false, false, true];
    let names = vec!["a".to_string(), "b".to_string()];
    let output = bool_func_parser::print_tabel(&table, &names, &"result".to_string());
    let lines = vec![
        "a b | result",
        "0 0 |   0",
        "0 1 |   0",
        "1 0 |   0",
        "1 1 |   1\n",
    ];
    assert_eq!(output, lines.join("\n"));

    let table = vec![true, true, true, true, true, true, false, true];
    let names = vec!["a".to_string(), "input".to_string(), "in_1".to_string()];
    let output = bool_func_parser::print_tabel(&table, &names, &"result".to_string());

    let lines = vec![
        "a input in_1 | result",
        "0   0     0  |   1",
        "0   0     1  |   1",
        "0   1     0  |   1",
        "0   1     1  |   1",
        "1   0     0  |   1",
        "1   0     1  |   1",
        "1   1     0  |   0",
        "1   1     1  |   1\n",
    ];

    assert_eq!(output, lines.join("\n"));
}

#[test]
fn generate_table_2d() {
    let table = vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
        vec![true, true, true],
    ];
    assert_eq!(bool_func_parser::generate_table_2d(3), table);

    assert_eq!(bool_func_parser::generate_table_2d(6).len(), 64);
}

#[test]
fn update_values() {
    let mut values = vec![false, true, false];
    assert_eq!(bool_func_parser::update_values(&mut values), true);
    assert_eq!(values, vec![false, true, true]);
    values = vec![true, true, true];
    assert_eq!(bool_func_parser::update_values(&mut values), false);
}

#[test]
fn get_names() {
    let input = vec![
        Token::Var("a".to_string()),
        Token::Var("b".to_string()),
        Token::Var("a".to_string()),
        Token::Var("c".to_string()),
        Token::Var("c".to_string()),
        Token::Var("d".to_string()),
    ];

    assert_eq!(
        bool_func_parser::get_names(&input),
        vec!["a", "b", "c", "d"]
    );
}

#[test]
fn single() {
    let output = vec![false, true];
    let parse = bool_func_parser::parse(&vec![Token::Var("a".to_string())]);
    let input = parse.unwrap();
    assert_eq!(input.len(), output.len());
    for i in 0..input.len() {
        assert_eq!(input[i], output[i], "at {}", i);
    }
}

#[test]
fn const_false() {
    let parse = bool_func_parser::parse(&vec![Token::Zero, Token::And, Token::One]);
    let output = parse.unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], false);
}

#[test]
fn const_true() {
    let parse = bool_func_parser::parse(&vec![Token::One, Token::And, Token::One]);
    let output = parse.unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], true);
}

#[test]
fn and() {
    let output = vec![false, false, false, true];
    let input = bool_func_parser::parse(&vec![
        Token::Var("a".to_string()),
        Token::And,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn xor() {
    let output = vec![false, true, true, false];
    let input = bool_func_parser::parse(&vec![
        Token::Var("a".to_string()),
        Token::Xor,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn or() {
    let output = vec![false, true, true, true];
    let input = bool_func_parser::parse(&vec![
        Token::Var("a".to_string()),
        Token::Or,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn not() {
    let output = vec![true, false];
    let input = bool_func_parser::parse(&vec![Token::Not, Token::Var("a".to_string())]);

    assert_eq!(input, Ok(output));
}

#[test]
fn identity() {
    let output = vec![false, true];
    let input = bool_func_parser::parse(&vec![Token::Var("a".to_string())]);

    assert_eq!(input, Ok(output));
}

#[test]
fn pares_complex_1() {
    // (a|b)&!c -> 0010 1010
    let output = vec![false, false, true, false, true, false, true, false];
    let input = bool_func_parser::parse(&vec![
        Token::Open,
        Token::Var("a".to_string()),
        Token::Or,
        Token::Var("b".to_string()),
        Token::Close,
        Token::And,
        Token::Not,
        Token::Var("c".to_string()),
    ]);
    assert_eq!(input, Ok(output));
}

#[test]
fn pares_complex_2() {
    //  (a&b&!c)  -> 0000 0010
    let output = vec![false, false, false, false, false, false, true, false];
    let input = bool_func_parser::parse(&vec![
        Token::Open,
        Token::Var("a".to_string()),
        Token::And,
        Token::Var("b".to_string()),
        Token::And,
        Token::Not,
        Token::Var("c".to_string()),
        Token::Close,
    ]);
    assert_eq!(input, Ok(output));
}

#[test]
fn pares_complex_3() {
    // !((a|b)&(c|!d)) -> 1111 0100 0100 0100
    let output = vec![
        true, true, true, true, false, true, false, false, false, true, false, false, false, true,
        false, false,
    ];
    let input = bool_func_parser::parse(&vec![
        Token::Not,
        Token::Open,
        Token::Open,
        Token::Var("a".to_string()),
        Token::Or,
        Token::Var("b".to_string()),
        Token::Close,
        Token::And,
        Token::Open,
        Token::Var("c".to_string()),
        Token::Or,
        Token::Not,
        Token::Var("d".to_string()),
        Token::Close,
        Token::Close,
    ]);
    assert_eq!(input, Ok(output));
}

#[test]
fn same_var() {
    // b & c | c -> 0101
    let output = vec![false, true, false, true];

    let func = vec![
        Token::Var("b".to_string()),
        Token::And,
        Token::Var("c".to_string()),
        Token::Or,
        Token::Var("c".to_string()),
    ];
    let input = bool_func_parser::parse(&func);
    let vars = bool_func_parser::get_names(&func);

    assert_eq!(vars, vec!["b".to_string(), "c".to_string()]);
    assert_eq!(input, Ok(output));
}
