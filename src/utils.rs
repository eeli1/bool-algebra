use crate::Token;
use std::usize;

/// parses all unique var names in the token stream (`func: &Vec<Token>`) and returns it in the same order the table was created
///
/// func: `a & b | a` -> `vec["a", "b"]`
///
/// ## Example
///
/// ```rust
/// use bool_algebra::*;
///
/// // a & b | a -> vec["a", "b"]
///
/// let input = vec![
///     Token::Var("a".to_string()),
///     Token::And,
///     Token::Var("b".to_string()),
///     Token::Or,
///     Token::Var("a".to_string()),
/// ];
/// let output = vec!["a".to_string(), "b".to_string()];
/// assert_eq!(get_names(&input), output);
/// ```
pub fn get_names(func: &Vec<Token>) -> Vec<String> {
    let mut vars = Vec::new();
    for f in func {
        match f.clone() {
            Token::Var(name) => {
                let mut in_var = false;
                for s in vars.clone() {
                    if s == name {
                        in_var = true;
                        break;
                    }
                }
                if !in_var {
                    vars.push(name);
                }
            }
            _ => (),
        }
    }
    vars.sort();
    vars
}

/// checks if the input function is a valid expression
/// returns Ok(()) if it's okay and Err(String) with an error message
pub fn validate_func(func: &Vec<Token>) -> Result<(), String> {
    // increments on '(' and decrements on ')' should never be -1. Exampel: (a) & b ) is invalid
    let mut count_parentheses = 0;
    // counts all binary operator (and, or, xor) and cheks if ther are enough identifiers. Exampel: !a & & b is invalid
    let mut count_binary = 0;
    // afer an identifier ther must be an operator. Exampel: a a & b is invalid
    let mut last_identifier = false;

    let mut count_identifier = 0;

    for f in func {
        match f {
            Token::ImplicAB => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::ImplicBA => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Eq => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::And => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Or => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Xor => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Nand => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Nor => {
                count_binary += 1;
                last_identifier = false;
            }
            Token::Not => (),
            Token::Open => count_parentheses += 1,
            Token::Close => {
                if count_parentheses == 0 {
                    return Err("too many closing parentheses".to_string());
                } else {
                    count_parentheses -= 1;
                }
            }
            Token::Var(name) => {
                if last_identifier {
                    return Err(format!("expected operator got {}", name));
                } else {
                    last_identifier = true;
                    count_identifier += 1;
                }
            }

            Token::One => {
                if last_identifier {
                    return Err(format!("expected operator got {}", "1"));
                } else {
                    last_identifier = true;
                    count_identifier += 1;
                }
            }
            Token::Zero => {
                if last_identifier {
                    return Err(format!("expected operator got {}", "0"));
                } else {
                    last_identifier = true;
                    count_identifier += 1;
                }
            }
        }
    }

    if count_binary != count_identifier - 1 {
        return Err(format!(
            "number of identifier doesn't match with the number of the binary operators"
        ));
    }
    if count_parentheses != 0 {
        return Err(format!(
            "number of open parentheses doesn't match with the number of closing parentheses"
        ));
    }
    Ok(())
}

/// checks if the input table is a valid
/// returns Ok(()) if it's okay and Err(String) with an error message
pub fn validate_tabel(table: &Vec<bool>, names: &Vec<String>) -> Result<(), String> {
    if table.len() != usize::pow(2, names.len() as u32) {
        Err(format!(
            "unexpected table len, expected 2^{} = {} got {}",
            names.len(),
            usize::pow(2, names.len() as u32),
            table.len()
        ))
    } else {
        Ok(())
    }
}

/// prints a functon
pub fn print_func(func: &Vec<Token>) -> String {
    let mut out = String::new();
    for token in func {
        out.push_str(&format!("{} ", token));
    }
    out.push_str("\n");
    out
}

/// retruns an error if the tabel is invalid
/// otherwise it retruns the table as a String
///
/// ## Example
///
/// ```rust
/// let table = vec![false, false, false, true];
/// let names = vec!["a".to_string(), "b".to_string()];
/// let output = bool_algebra::print_tabel(&table, &names, &"result".to_string());
/// let lines = vec![
///     "a b | result",
///     "0 0 |   0",
///     "0 1 |   0",
///     "1 0 |   0",
///     "1 1 |   1\n",
/// ];
/// assert_eq!(output, lines.join("\n"));
/// ```
pub fn print_tabel(table: &Vec<bool>, names: &Vec<String>, result_name: &String) -> String {
    if let Err(err) = validate_tabel(table, names) {
        return format!("{}", err);
    }

    fn get_offset(name: &String) -> (usize, usize) {
        let len = name.len() - 1;
        let right = len / 2;
        let left = len - right;
        (left, right)
    }

    fn print_offset(offset: (usize, usize), value: bool) -> String {
        let mut out = String::new();
        for _ in 0..offset.0 {
            out.push(' ');
        }
        if value {
            out.push('1');
        } else {
            out.push('0');
        }
        for _ in 0..offset.1 {
            out.push(' ');
        }
        out
    }

    let mut out = String::new();

    for name in names {
        out.push_str(&format!("{} ", name));
    }
    out.push_str(&format!("| {}\n", result_name));

    let offset: Vec<(usize, usize)> = names.iter().map(get_offset).collect();
    let result_offset = get_offset(result_name).1;

    let mut values = vec![false; names.len()];
    let mut index = 0;

    loop {
        for (i, &value) in values.iter().enumerate() {
            out.push_str(&print_offset(offset[i], value));
            out.push(' ');
        }
        out.push_str(&format!("| "));
        for _ in 0..result_offset {
            out.push(' ');
        }

        if table[index] {
            out.push('1');
        } else {
            out.push('0');
        }
        out.push('\n');

        if !update_values(&mut values) {
            break;
        }
        index += 1;
    }

    out
}

/// binary add one 0010 -> 0011 -> 0100 ...
/// returns false if all combiantion have been teste eg. 1111
///
/// ## Example
///
/// ```rust
/// let mut values = vec![false, true, false];
/// assert_eq!(bool_algebra::update_values(&mut values), true);
/// assert_eq!(values, vec![false, true ,true]);
/// ```
pub fn update_values(values: &mut Vec<bool>) -> bool {
    for i in (0..values.len()).rev() {
        values[i] = !values[i];
        if values[i].clone() == true {
            return true;
        }
    }
    return false;
}

/// generates a table_2d that counts in binary form 0 until every bit is one
///
/// ## Example
///
/// ```rust
/// let table = vec![
///   vec![false, false, false],
///   vec![false, false, true],
///   vec![false, true, false],
///   vec![false, true, true],
///   vec![true, false, false],
///   vec![true, false, true],
///   vec![true, true, false],
///   vec![true, true, true],
/// ];
/// assert_eq!(bool_algebra::generate_table_2d(3), table);
/// ```
pub fn generate_table_2d(len: usize) -> Vec<Vec<bool>> {
    let mut table_2d = Vec::new();
    let mut values = vec![false; len];
    table_2d.push(values.clone());

    while update_values(&mut values) {
        table_2d.push(values.clone());
    }
    table_2d
}

/// converts a boolean array to a u32 
/// 
///  ## Example
///
/// ```rust
/// assert_eq!(bool_algebra::bool_to_u32(vec![false, true, true]), 3); // 011 -> 3
/// assert_eq!(bool_algebra::bool_to_u32(vec![true, false, false]), 4); // 100 -> 4
/// ```
pub fn bool_to_u32(binary: Vec<bool>) -> u32 {
    let mut num = 0;
    for (i, &b) in binary.iter().rev().enumerate() {
        if b {
            num += 2_u32.pow(i as u32);
        }
    }
    num
}
