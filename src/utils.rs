use crate::Token;
use std::usize;

/// parses all unique var names in the token stream (`func: &Vec<Token>`) and returns it in the same order the table was created
///
/// func: `a & b | a` -> `vec["a", "b"]`
///
/// # example:
///
/// ```rust
/// use bool_func_parser::*;
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
pub fn validate_tabel(table: Vec<bool>, names: Vec<String>) -> Result<(), String> {
    if table.len() != usize::pow(2, names.len() as u32) {
        Err(format!("unexpected table len, expected 2^{} = {} got {}", names.len() ,usize::pow(2, names.len() as u32) ,table.len()))
    } else {
        Ok(())
    }
}
