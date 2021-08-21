use super::*;

mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let func = vec![Token::Var {
            name: "a".to_string(),
        }];
        let mut values = Vec::<bool>::new();
        let mut lookup = HashMap::<Token, *mut bool>::new();
        init_lookup_values(get_names(&func), &mut lookup, &mut values);
        let mut temp = vec![false, true];
        let const_bool = unsafe { vec![temp.as_mut_ptr().add(0), temp.as_mut_ptr().add(1)] };
        let tree = Node::build_tree(func.clone(), &lookup, &const_bool);
        assert_eq!(Node::eval(tree), Some(false));
        if let Some(ptr) = lookup.get(&Token::Var {
            name: "a".to_string(),
        }) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), false);
            }
        }
        assert_eq!(update_values(&mut values), true);
        assert_eq!(Node::eval(tree), Some(true));
        if let Some(ptr) = lookup.get(&Token::Var {
            name: "a".to_string(),
        }) {
            unsafe {
                assert_eq!(format!("{:?}", ptr), format!("{:?}", values.as_ptr()));
                assert_eq!(ptr.read_volatile(), true);
            }
        }
        assert_eq!(update_values(&mut values), false);
    }

    #[test]
    fn test_lookup_values_2() {
        let func = vec![
            Token::Var {
                name: "a".to_string(),
            },
            Token::Var {
                name: "b".to_string(),
            },
        ];
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
            Token::Var {
                name: "a".to_string(),
            },
            Token::Var {
                name: "b".to_string(),
            },
            Token::Var {
                name: "c".to_string(),
            },
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
            Token::Var {
                name: "a".to_string(),
            },
            Token::And,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
        ];
        assert_eq!(is_parentheses(&input), true);
    }

    #[test]
    fn is_parentheses_false() {
        let input = vec![
            Token::Open,
            Token::Var {
                name: "a".to_string(),
            },
            Token::Close,
            Token::And,
            Token::Open,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
        ];
        assert_eq!(is_parentheses(&input), false);
    }

    #[test]
    fn test_split_index() {
        // (a | b) & c
        let input = vec![
            Token::Open,
            Token::Var {
                name: "a".to_string(),
            },
            Token::Or,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
            Token::And,
            Token::Var {
                name: "c".to_string(),
            },
        ];

        assert_eq!(Bundle::split_index(&input), 5);
    }

    #[test]
    fn test_split_not() {
        // !(a & b)
        let input = vec![
            Token::Not,
            Token::Open,
            Token::Var {
                name: "a".to_string(),
            },
            Token::And,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
        ];

        let bundle = Bundle::split(&input);
        assert_eq!(bundle.center, Token::Not);
        assert_eq!(
            bundle.left,
            Some(vec![
                Token::Open,
                Token::Var {
                    name: "a".to_string(),
                },
                Token::And,
                Token::Var {
                    name: "b".to_string(),
                },
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
            Token::Var {
                name: "a".to_string(),
            },
            Token::Close,
            Token::And,
            Token::Open,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
        ];

        let bundle = Bundle::split(&input);
        assert_eq!(bundle.center, Token::And);
        assert_eq!(
            bundle.left,
            Some(vec![
                Token::Open,
                Token::Var {
                    name: "a".to_string(),
                },
                Token::Close,
            ])
        );
        assert_eq!(
            bundle.right,
            Some(vec![
                Token::Open,
                Token::Var {
                    name: "b".to_string(),
                },
                Token::Close,
            ])
        );
    }

    #[test]
    fn test_split_parentheses() {
        let bundle = Bundle::split(&vec![
            Token::Open,
            Token::Var {
                name: "a".to_string(),
            },
            Token::And,
            Token::Var {
                name: "b".to_string(),
            },
            Token::Close,
        ]);

        assert_eq!(bundle.center, Token::And);
        assert_eq!(
            bundle.left,
            Some(vec![Token::Var {
                name: "a".to_string(),
            },])
        );

        assert_eq!(
            bundle.right,
            Some(vec![Token::Var {
                name: "b".to_string(),
            },])
        );
    }
}
