# boolean function parser

parsers boolean function like `a & b` in to `0001`

## parse

`pub fn parse(func: &Vec<Token>) -> Option<Vec<bool>> {...}`

this is the main parse function
  
this function parses the token stream (`func: &Vec<Token>`) into a compressed boolean table `Option<Vec<bool>>` and returns `None` if it can't parse the function

func: `a & b`

compressed table: `0001`

full table:

```#
a b | result
0 0 |   0
0 1 |   0
1 0 |   0
0 1 |   1
```

the compressed table is just result read vertically

### example parse()

```rust
use bool_func_parser::*;

// a & b -> 0001

let output = vec![false, false, false, true];
let input = parse(&vec![
    Token::Var {
        name: "a".to_string(),
    },
    Token::And,
    Token::Var {
        name: "b".to_string(),
    },
 ]);

assert_eq!(input, Some(output));
```

## get names

`pub fn get_names(func: &Vec<Token>) -> Vec<String> {`

parses all unique var names in the token stream (`func: &Vec<Token>`) and returns it in the same order the table was created

func: `a & b | a` -> `vec["a", "b"]`

### example get_names()

```rust
use bool_func_parser::*;
// a & b | a -> vec["a", "b"]

let input = vec![
    Token::Var {
        name: "a".to_string(),
    },
    Token::And,
    Token::Var {
        name: "b".to_string(),
    },
    Token::Or,
    Token::Var {
        name: "a".to_string(),
    },
];
let output = vec!["a".to_string(), "b".to_string()];
assert_eq!(get_names(&input), output);
```

## how it works

the problem of evaluating expressions is the order of operation

for example, multiplication comes for addition or in our case and comes for or

and if you add parentheses the problem does only become more complicated

and because you have to parse the expression for every combination

this means if you have an expression with 2 variables

you need to evaluate it 4 times to handle all combinations

or to say it more abstractly for n unique variables you need to evaluate the expression 2^n times

so in this case it makes sense to use a binary tree

you can build the tree in O(n²) which you have to do only once

and to evaluate the expression you can do it with O(log(n))

so the whole pase function has a O(n) of `n² * 2^(log(n)) -> n² * n -> n³` so O(n³)

(It's been a while since the last time I did O(n) Complexity so take these values with a grain of salt)

### example !(a & b) ^ c

generate tree from function: `!(a & b) | c`

and create a lookup table (lt) then map the leaf node to the lookup table (the table has as many entries as there are unique variables)

```#
             | 
          /     \
        !         c
        |         
        &         
      /   \       
    a       b     
    :       :     :
  lt[0]  lt[1]  lt[2]
```

to evaluate the tree you only have to set the values in the lookup table (lt) and then traverse the tree then update and repeated until you have tried every value

to optimize this you can also prune the tree so you don't have to consider every value

for example if c -> lt[2] turns out to be one you know the whole expression must be one because one ord with any other expression is always one

full tabel for `!(a & b) | c`:

```#
a b c | result
0 0 0 |   1
0 0 1 |   1
0 1 0 |   1
0 1 1 |   1
1 0 0 |   1
1 0 1 |   1
1 1 0 |   0
1 1 1 |   1
```
