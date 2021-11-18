use crate::{update_values, validate_func, validate_tabel, Token};

/// generates the [disjunctive normal form] (DNF)
///
/// [disjunctive normal form]:https://en.wikipedia.org/wiki/Disjunctive_normal_form
///
/// ## Example
///
/// ```rust
/// use bool_algebra::Token;
/// let table = vec![false, false, false, true];
/// let names = vec!["a".to_string(), "b".to_string()];
/// let dnf = vec![
///     Token::Open,
///     Token::Var("a".to_string()),
///     Token::And,
///     Token::Var("b".to_string()),
///     Token::Close,
/// ];
/// assert_eq!(bool_algebra::dnf(&table, &names, true), Ok(dnf));
/// ```
pub fn dnf(
    table: &Vec<bool>,
    names: &Vec<String>,
    with_parentheses: bool,
) -> Result<Vec<Token>, String> {
    validate_tabel(table, names)?;
    let names: Vec<Token> = names.iter().map(|name| Token::Var(name.clone())).collect();

    let mut dnf = Vec::new();

    let mut values = vec![false; names.len()];
    let mut index = 0;
    loop {
        if table[index] {
            if dnf.len() != 0 {
                dnf.push(Token::Or);
            }
            if with_parentheses {
                dnf.push(Token::Open);
            }
            for (i, &value) in values.iter().enumerate() {
                if !value {
                    dnf.push(Token::Not);
                }
                dnf.push(names[i].clone());
                dnf.push(Token::And);
            }
            // pop last operator (Token::And)
            dnf.pop();
            if with_parentheses {
                dnf.push(Token::Close);
            }
        }
        if !update_values(&mut values) {
            break;
        }
        index += 1;
    }

    validate_func(&dnf)?;
    Ok(dnf)
}
