use bool_algebra::parse;
use bool_algebra::Token;

#[test]
fn single() {
    let output = vec![false, true];
    let parse = parse(&vec![Token::Var("a".to_string())]);
    let input = parse.unwrap();
    assert_eq!(input.len(), output.len());
    for i in 0..input.len() {
        assert_eq!(input[i], output[i], "at {}", i);
    }
}

#[test]
fn const_false() {
    let parse = parse(&vec![Token::Zero, Token::And, Token::One]);
    let output = parse.unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], false);
}

#[test]
fn const_true() {
    let parse = parse(&vec![Token::One, Token::And, Token::One]);
    let output = parse.unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], true);
}

#[test]
fn and() {
    let output = vec![false, false, false, true];
    let input = parse(&vec![
        Token::Var("a".to_string()),
        Token::And,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn xor() {
    let output = vec![false, true, true, false];
    let input = parse(&vec![
        Token::Var("a".to_string()),
        Token::Xor,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn or() {
    let output = vec![false, true, true, true];
    let input = parse(&vec![
        Token::Var("a".to_string()),
        Token::Or,
        Token::Var("b".to_string()),
    ]);

    assert_eq!(input, Ok(output));
}

#[test]
fn not() {
    let output = vec![true, false];
    let input = parse(&vec![Token::Not, Token::Var("a".to_string())]);

    assert_eq!(input, Ok(output));
}

#[test]
fn identity() {
    let output = vec![false, true];
    let input = parse(&vec![Token::Var("a".to_string())]);

    assert_eq!(input, Ok(output));
}

#[test]
fn pares_complex_1() {
    // (a|b)&!c -> 0010 1010
    let output = vec![false, false, true, false, true, false, true, false];
    let input = parse(&vec![
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
    let input = parse(&vec![
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
    let input = parse(&vec![
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
    let input = parse(&func);
    let vars = bool_algebra::get_names(&func);

    assert_eq!(vars, vec!["b".to_string(), "c".to_string()]);
    assert_eq!(input, Ok(output));
}
