use bool_algebra::{parse_count, parse_fill, parse_full};

fn str2_bool(input: &str) -> Vec<bool> {
    let mut result = Vec::new();
    for c in input.chars() {
        if c == '1' {
            result.push(true);
        } else if c == '0' {
            result.push(false);
        }
    }
    result
}

#[test]
fn test_fill() {
    assert_eq!(
        parse_fill(2, 1, str2_bool("111000010100"), false),
        Ok(vec![vec![false, false, false, true]])
    );
    assert_eq!(
        parse_fill(2, 1, str2_bool("111"), false),
        Ok(vec![vec![false, false, false, true]])
    );
}

#[test]
fn test_full_medium() {
    let table_stream = str2_bool(
        "
        01 101
        00 000
        11 110
        10 101",
    );

    assert_eq!(
        parse_full(2, 3, table_stream),
        Ok(vec![
            str2_bool("0111"),
            str2_bool("0001"),
            str2_bool("0110")
        ]),
    );
}

#[test]
fn test_full_small() {
    let table_stream = str2_bool(
        "0 0
        1 1",
    );

    assert_eq!(parse_full(1, 1, table_stream), Ok(vec![str2_bool("01"),]),);
}

#[test]
fn test_full_large() {
    let table_stream = str2_bool(
        "
        00000 01
        00001 01
        00010 01
        00011 00
        00100 10
        00101 10
        00110 01
        00111 10
        01000 11
        01001 00
        01010 10
        01011 10
        01100 00
        01101 00
        01110 00
        01111 11
        10000 01
        10001 11
        10010 01
        10011 01
        10100 00
        10101 00
        10110 00
        10111 11
        11000 11
        11001 11
        11010 01
        11011 01
        11100 10
        11101 10
        11111 10
        11110 10",
    );

    assert_eq!(
        parse_full(5, 2, table_stream),
        Ok(vec![
            str2_bool("00001101101100010100000111001111"),
            str2_bool("11100010100000011111000111110000")
        ]),
    );
}

#[test]
fn test_count_medium() {
    let table_stream = str2_bool(
        "
            0111
            0001
            0110",
    );
    assert_eq!(
        parse_count(2, 3, table_stream, true),
        Ok(vec![
            str2_bool("0111"),
            str2_bool("0001"),
            str2_bool("0110")
        ]),
    );

    let table_stream = str2_bool(
        "
    000
    101
    101
    110",
    );
    assert_eq!(
        parse_count(2, 3, table_stream, false),
        Ok(vec![
            str2_bool("0111"),
            str2_bool("0001"),
            str2_bool("0110")
        ]),
    );
}

#[test]
fn test_count_small() {
    assert_eq!(
        parse_count(1, 1, str2_bool("01"), true),
        Ok(vec![str2_bool("01"),]),
    );
}

#[test]
fn test_count_large() {
    let table_stream = str2_bool(
        "
        00001101101100010100000111001111
        11100010100000011111000111110000",
    );
    assert_eq!(
        parse_count(5, 2, table_stream, true),
        Ok(vec![
            str2_bool("00001101101100010100000111001111"),
            str2_bool("11100010100000011111000111110000")
        ]),
    );

    let table_stream = str2_bool(
        "
            01
            01
            01
            00
            10
            10
            01
            10
            11
            00
            10
            10
            00
            00
            00
            11
            01
            11
            01
            01
            00
            00
            00
            11
            11
            11
            01
            01
            10
            10
            10
            10",
    );

    assert_eq!(
        parse_count(5, 2, table_stream, false),
        Ok(vec![
            str2_bool("00001101101100010100000111001111"),
            str2_bool("11100010100000011111000111110000")
        ]),
    );
}
