use crate::{get_names, validate_func, Token, update_values};
use std::collections::HashMap;
use std::usize;

/// this is the main parse function
///  
/// this function parses the token vec (`func: &Vec<Token>`) into a compressed boolean table `Ok` and returns `Err` with an error massage if it can't parse the function
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
/// 1 1 |   1
/// ```
///
/// the compressed table is just result read vertically
///
/// # example:
///
/// ```rust
/// use bool_algebra::*;
///
/// // a & b -> 0001
///
/// let output = vec![false, false, false, true];
/// let input = parse(&vec![
///     Token::Var("a".to_string()),
///     Token::And,
///     Token::Var("b".to_string()),
///  ]);
///
/// assert_eq!(input, Ok(output));
/// ```
pub fn parse(func: &Vec<Token>) -> Result<Vec<bool>, String> {
    validate_func(func)?;

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
            return Err(format!("something went wrong cannot evaluate expression"));
        }
        if !update_values(&mut values) {
            return Ok(result);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let func = vec![Token::Var("a".to_string())];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<Token, *mut bool>::new();
        init_lookup_values(get_names(&func), &mut lookup, &mut values);
        let mut temp = vec![false, true];
        let const_bool = unsafe { vec![temp.as_mut_ptr().add(0), temp.as_mut_ptr().add(1)] };
        let tree = Node::build_tree(func.clone(), &lookup, &const_bool);
        assert_eq!(Node::eval(tree), Some(false));
        if let Some(ptr) = lookup.get(&Token::Var("a".to_string())) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), false);
            }
        }
        assert_eq!(update_values(&mut values), true);
        assert_eq!(Node::eval(tree), Some(true));
        if let Some(ptr) = lookup.get(&Token::Var("a".to_string())) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), true);
            }
        }
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_lookup_values_2() {
        let func = vec![Token::Var("a".to_string()), Token::Var("b".to_string())];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<Token, *mut bool>::new();
        init_lookup_values(get_names(&func), &mut lookup, &mut values);

        assert_eq!(values, vec![false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true]);
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_lookup_values_3() {
        let func = vec![
            Token::Var("a".to_string()),
            Token::Var("b".to_string()),
            Token::Var("c".to_string()),
        ];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<Token, *mut bool>::new();
        init_lookup_values(get_names(&func), &mut lookup, &mut values);

        assert_eq!(values, vec![false, false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![false, true, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, false, true]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true, false]);
        assert_eq!(update_values(&mut values), true);
        assert_eq!(values, vec![true, true, true]);
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn is_parentheses_true() {
        let input = vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Var("b".to_string()),
            Token::Close,
        ];
        assert_eq!(is_parentheses(&input), true);
    }

    #[test]
    fn is_parentheses_false() {
        let input = vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::Close,
            Token::And,
            Token::Open,
            Token::Var("b".to_string()),
            Token::Close,
        ];
        assert_eq!(is_parentheses(&input), false);
    }

    #[test]
    fn test_split_index() {
        // (a | b) & c
        let input = vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::Or,
            Token::Var("b".to_string()),
            Token::Close,
            Token::And,
            Token::Var("c".to_string()),
        ];

        assert_eq!(Bundle::split_index(&input), 5);
    }

    #[test]
    fn test_split_not() {
        // !(a & b)
        let input = vec![
            Token::Not,
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Var("b".to_string()),
            Token::Close,
        ];

        let bundle = Bundle::split(&input);
        assert_eq!(bundle.center, Token::Not);
        assert_eq!(
            bundle.left,
            Some(vec![
                Token::Open,
                Token::Var("a".to_string()),
                Token::And,
                Token::Var("b".to_string()),
                Token::Close,
            ])
        );
        assert_eq!(bundle.right, None);
    }

    // this is a very imortant function to buid the tree. It splits the function in to 3 parts Example: a & b | (c | d) -> a & b, |, (c | d)
    // this works with any arbitrarily complicated function
    // fn split

    #[test]
    fn test_split_operator() {
        let input = vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::Close,
            Token::And,
            Token::Open,
            Token::Var("b".to_string()),
            Token::Close,
        ];

        let bundle = Bundle::split(&input);
        assert_eq!(bundle.center, Token::And);
        assert_eq!(
            bundle.left,
            Some(vec![Token::Open, Token::Var("a".to_string()), Token::Close,])
        );
        assert_eq!(
            bundle.right,
            Some(vec![Token::Open, Token::Var("b".to_string()), Token::Close,])
        );
    }

    #[test]
    fn test_split_parentheses() {
        let bundle = Bundle::split(&vec![
            Token::Open,
            Token::Var("a".to_string()),
            Token::And,
            Token::Var("b".to_string()),
            Token::Close,
        ]);

        assert_eq!(bundle.center, Token::And);
        assert_eq!(bundle.left, Some(vec![Token::Var("a".to_string()),]));

        assert_eq!(bundle.right, Some(vec![Token::Var("b".to_string()),]));
    }
}
