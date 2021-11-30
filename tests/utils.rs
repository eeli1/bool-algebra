use bool_algebra::Token;

#[test]
fn dnf_smale() {
    let table = vec![false, false, false, true];
    let names = vec!["a".to_string(), "b".to_string()];
    let dnf = vec![
        Token::Open,
        Token::Var("a".to_string()),
        Token::And,
        Token::Var("b".to_string()),
        Token::Close,
    ];
    assert_eq!(bool_algebra::dnf(&table, &names, true), Ok(dnf));

    let dnf = vec![
        Token::Var("a".to_string()),
        Token::And,
        Token::Var("b".to_string()),
    ];
    assert_eq!(bool_algebra::dnf(&table, &names, false), Ok(dnf));
}

#[test]
fn dnf_long() {
    let table = vec![true, true, false, false, true, true, false, false];
    let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let dnf = vec![
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
    ];
    assert_eq!(bool_algebra::dnf(&table, &names, true), Ok(dnf));

    let dnf = vec![
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
    ];
    assert_eq!(bool_algebra::dnf(&table, &names, false), Ok(dnf));
}

#[test]
fn print_tabel() {
    let table = vec![false, false, false, true];
    let names = vec!["a".to_string(), "b".to_string()];
    let output = bool_algebra::print_tabel(&table, &names, &"result".to_string());
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
    let output = bool_algebra::print_tabel(&table, &names, &"result".to_string());

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
    assert_eq!(bool_algebra::generate_table_2d(3), table);

    assert_eq!(bool_algebra::generate_table_2d(6).len(), 64);
}

#[test]
fn update_values() {
    let mut values = vec![false, true, false];
    assert_eq!(bool_algebra::update_values(&mut values), true);
    assert_eq!(values, vec![false, true, true]);
    values = vec![true, true, true];
    assert_eq!(bool_algebra::update_values(&mut values), false);
}

#[test]
fn get_names() {
    let input = vec![
        Token::Var("d".to_string()),
        Token::Var("c".to_string()),
        Token::Var("a".to_string()),
        Token::Var("b".to_string()),
        Token::Var("a".to_string()),
        Token::Var("c".to_string()),
    ];

    assert_eq!(bool_algebra::get_names(&input), vec!["a", "b", "c", "d"]);
}

#[test]
fn bool_to_u32() {
    assert_eq!(bool_algebra::bool_to_u32(vec![false, true, true]), 3);
    assert_eq!(bool_algebra::bool_to_u32(vec![true, false, false]), 4);
}
