use std::collections::HashMap;
use std::usize;

mod test_internal;

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

/// this is the main parse function
///  
/// this function parses the token stream (`func: &Vec<Token>`) into a compressed boolean table `Option<Vec<bool>>` and returns `None` if it can't parse the function
///
/// func: `a & b`
///
/// compressed table: `0001`
///
/// full table:
///
/// ```#
/// a b | result
/// 0 0 |   0
/// 0 1 |   0
/// 1 0 |   0
/// 0 1 |   1
/// ```
///
/// the compressed table is just result read vertically
///
/// # example:
///
/// ```rust
/// use bool_func_parser::*;
///
/// // a & b -> 0001
///
/// let output = vec![false, false, false, true];
/// let input = parse(&vec![
///     Token::Var("a".to_string())),
///     Token::And,
///     Token::Var("b".to_string()),
///  ]);
///
/// assert_eq!(input, Some(output));
/// ```
pub fn parse(func: &Vec<Token>) -> Option<Vec<bool>> {
    if validate(func).is_err() {
        return None;
    }

    let names = get_names(func);
    let len = usize::pow(2, names.len() as u32);
    let mut values = Vec::with_capacity(len);
    let mut lookup = HashMap::<Token, *mut bool>::with_capacity(len);
    init_lookup_values(names, &mut lookup, &mut values);

    // this const bool is for the Token::One and Token::Zero thes valus don't get updated
    let mut temp = vec![false, true];
    let const_bool = unsafe { vec![temp.as_mut_ptr().add(0), temp.as_mut_ptr().add(1)] };

    let tree = Node::build_tree(func.clone(), &lookup, &const_bool);
    let mut result = Vec::with_capacity(len);

    loop {
        if let Some(value) = Node::eval(tree) {
            result.push(value);
        } else {
            return None;
        }
        if !update_values(&mut values) {
            return Some(result);
        }
    }
}

/// checks if the input function is a valid expression
/// returns Ok(()) if it's okay and Err(String) with an error message
pub fn validate(func: &Vec<Token>) -> Result<(), String> {
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

/// the lookup table is to build the tree if the node is a leaf node it stores a pointer to a boolean array
/// this array gets updated with the function `update_values()`
/// note tear are 2 constant values in the lookup table which don't change
fn init_lookup_values(
    names: Vec<String>,
    lookup: &mut HashMap<Token, *mut bool>,
    values: &mut Vec<bool>,
) {
    for name in names {
        values.push(false);
        unsafe {
            let ptr = values.as_mut_ptr().add(values.len() - 1);
            lookup.entry(Token::Var(name)).or_insert(ptr);
        }
    }
}

/// binary add one 0010 -> 0011 -> 0100 ...
/// returns false if all combiantion have been teste eg. 1111
fn update_values(values: &mut Vec<bool>) -> bool {
    for i in (0..values.len()).rev() {
        values[i] = !values[i];
        if values[i].clone() == true {
            return true;
        }
    }
    return false;
}

// low precedence -> high in the tree must be split first
fn precedence_of(bool_func: &Token) -> usize {
    match bool_func {
        Token::Or => 0,
        Token::Xor => 1,
        Token::And => 2,
        Token::Not => 3,
        _ => 0xff,
    }
}

/// Example (a & b) -> true, (a) & (b) -> false
fn is_parentheses(func: &Vec<Token>) -> bool {
    if func[0] != Token::Open {
        return false;
    }
    if func.last() != Some(&Token::Close) {
        return false;
    }

    let mut counter = 1;
    for i in 1..(func.len() - 1) {
        if counter == 0 {
            return false;
        }
        if func[i] == Token::Open {
            counter += 1;
        }
        if func[i] == Token::Close {
            counter -= 1;
        }
    }
    true
}

#[derive(Debug)]
struct Bundle {
    left: Option<Vec<Token>>,
    right: Option<Vec<Token>>,
    center: Token,
}

impl Bundle {
    fn split_not(func: &Vec<Token>) -> Self {
        let mut left = Vec::new();
        for i in 1..func.len() {
            left.push(func[i].clone());
        }

        Self {
            left: Some(left),
            center: func[0].clone(),
            right: None,
        }
    }

    fn split_operator(func: &Vec<Token>) -> Self {
        let index = Self::split_index(func);

        // if it is 0 it must be not or an error
        if index == 0 {
            if func[0].clone() == Token::Not {
                return Self::split_not(func);
            }
            unreachable!();
        }

        let mut left = Vec::new();
        let mut right = Vec::new();
        let center = func[index].clone();
        for i in 0..index {
            left.push(func[i].clone());
        }
        for i in (index + 1)..func.len() {
            right.push(func[i].clone());
        }

        Self {
            left: Some(left),
            right: Some(right),
            center,
        }
    }

    /// split_index: gives the index of the operator by which the expression must be split
    /// Exampel: "(a|b)&c" the function would return 5 witch is the index of the char '&'
    fn split_index(func: &Vec<Token>) -> usize {
        let mut operator_index = 0;
        let mut operator_score = 0xff;
        let mut parentheses = 0;

        for (i, t) in func.iter().enumerate() {
            if parentheses != 0 {
                if t == &Token::Close {
                    parentheses -= 1;
                }
                if t == &Token::Open {
                    parentheses += 1;
                }
                continue;
            }
            if t == &Token::Open {
                parentheses += 1;
            }

            if precedence_of(&t) < operator_score {
                operator_score = precedence_of(&t);
                operator_index = i;
                if operator_score == 0 {
                    break;
                }
            }
        }

        return operator_index;
    }

    /// this is a very imortant function to buid the tree. It splits the function in to 3 parts Example: a & b | (c | d) -> a & b, |, (c | d)
    /// this works with any arbitrarily complicated function
    fn split(func: &Vec<Token>) -> Self {
        // is leavnode
        if func.len() == 1 {
            return Self {
                left: None,
                right: None,
                center: func[0].clone(),
            };
        }

        if is_parentheses(func) {
            return Self::split_parentheses(func);
        }

        return Self::split_operator(func);
    }

    /// remove fist and last item and calls split
    fn split_parentheses(func: &Vec<Token>) -> Self {
        let mut result = Vec::new();
        for i in 1..(func.len() - 1) {
            result.push(func[i].clone());
        }
        return Self::split(&result);
    }
}

/// the node struct is for the tree
#[derive(Debug)]
struct Node {
    left: Option<*mut Node>,
    right: Option<*mut Node>,
    value: Option<*mut bool>,
    operator: Option<Token>,
}

impl Node {
    pub fn build_tree(
        func: Vec<Token>,
        lookup: &HashMap<Token, *mut bool>,
        const_bool: &Vec<*mut bool>,
    ) -> *mut Self {
        let bundle = Bundle::split(&func);

        // is binery operator (and, or, xor)
        if let (Some(right), Some(left)) = (bundle.right.clone(), bundle.left.clone()) {
            let node = Box::new(Node {
                left: Some(Node::build_tree(left.clone(), lookup, const_bool)),
                right: Some(Node::build_tree(right.clone(), lookup, const_bool)),
                value: None,
                operator: Some(bundle.center.clone()),
            });

            return Box::into_raw(node);
        }

        // is unery operator (not)
        if let Some(left) = bundle.left.clone() {
            let node = Box::new(Node {
                left: Some(Node::build_tree(left.clone(), lookup, const_bool)),
                right: None,
                value: None,
                operator: Some(bundle.center.clone()),
            });

            return Box::into_raw(node);
        }

        // is leaf node
        let value;
        if bundle.center == Token::Zero {
            value = Some(const_bool[0]);
        } else if bundle.center == Token::One {
            value = Some(const_bool[1]);
        } else {
            value = Some(*lookup.get(&bundle.center).unwrap());
        }

        let node = Box::new(Node {
            left: None,
            right: None,
            value,
            operator: None,
        });
        Box::into_raw(node)
    }

    pub fn eval(node_ptr: *mut Node) -> Option<bool> {
        // can do unsafe because node_ptr != null
        unsafe {
            // if levenode then return value from lookuptable
            if let Some(value) = (*node_ptr).value {
                return Some(*value);
            }

            // if is not => unary operator only left side
            if (*node_ptr).operator == Some(Token::Not) {
                let left_node = (*node_ptr).left?;
                let left = Node::eval(left_node)?;
                return Some(!left);
            }

            // check if tree has left and right node get value recursively
            // for pruning check right first sckause left is longer
            let left_node = (*node_ptr).left?;
            let right_node = (*node_ptr).right?;
            let operator = (*node_ptr).operator.clone()?;
            match operator {
                Token::And => {
                    let right = Node::eval(right_node)?;
                    if right == false {
                        return Some(false);
                    } else {
                        let left = Node::eval(left_node)?;
                        return Some(left & right);
                    }
                }
                Token::Or => {
                    let right = Node::eval(right_node)?;
                    if right == true {
                        return Some(true);
                    } else {
                        let left = Node::eval(left_node)?;
                        return Some(left | right);
                    }
                }
                Token::Xor => {
                    let right = Node::eval(right_node)?;
                    let left = Node::eval(left_node)?;
                    return Some(left ^ right);
                }
                _ => {
                    return None;
                }
            }
        }
    }
}
