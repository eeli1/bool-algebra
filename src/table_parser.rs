/// convert a full tabel to a comprest table 
/// ```#
/// a b | and
/// 0 0 |  0
/// 1 0 |  0
/// 1 1 |  1
/// 0 1 |  0
/// ```
/// 
/// => 0001
///
/// Note the rows don't have to be in order 
/// 
/// ## Example
/// ```rust
/// assert_eq!(
///   bool_algebra::parse_full(1, 1, vec![false, false, true, true]),
///   Ok(vec![vec![false, true],]),
///  );
/// ```
pub fn parse_full(
    in_len: usize,
    out_len: usize,
    table: Vec<bool>,
) -> Result<Vec<Vec<bool>>, String> {
    if table.len() != pow2(in_len) * (out_len + in_len) {
        return Err("incorrect table shape".to_string());
    }
  
    return parse_fill(in_len, out_len, table, false);
}

pub fn parse_count(
    in_len: usize,
    out_len: usize,
    table: Vec<bool>,
    count_vertical: bool,
) -> Result<Vec<Vec<bool>>, String> {
    if table.len() != pow2(in_len) * out_len {
        return Err("incorrect table shape".to_string());
    }
    let mut result = Vec::new();

    if count_vertical {
        for i in 0..out_len {
            let mut temp = Vec::new();
            for j in 0..pow2(in_len) {
                temp.push(table[i * pow2(in_len) + j]);
            }
            result.push(temp);
        }
    } else {
        for _ in 0..out_len {
            result.push(Vec::new());
        }
        for i in 0..table.len() {
            result[i % out_len].push(table[i]);
        }
    }

    Ok(result)
}

pub fn parse_fill(
    in_len: usize,
    out_len: usize,
    table: Vec<bool>,
    fill: bool,
) -> Result<Vec<Vec<bool>>, String> {
    let vec_2d = split_rows(table, in_len + out_len);
    let result = match_table(vec_2d, out_len, fill);

    Ok(result)
}

fn pow2(exp: usize) -> usize {
    let mut result = 1;
    for _ in 0..exp {
        result *= 2;
    }
    result
}

fn split_rows(vec: Vec<bool>, len: usize) -> Vec<Vec<bool>> {
    let mut rows = Vec::new();
    let mut index = 0;

    while vec.len() != index * len {
        for i in 0..len {
            if i == 0 {
                rows.push(vec![vec[i + len * index]]);
            } else {
                rows[index].push(vec[i + len * index]);
            }
        }
        index += 1;
    }

    rows
}

fn bool2_int(vec: Vec<bool>) -> usize {
    let mut result = 0;
    for i in 0..vec.len() {
        if vec[i] {
            result += pow2(vec.len() - i - 1);
        }
    }
    result
}

fn match_line(vec: Vec<bool>, len: usize) -> (usize, Vec<bool>) {
    let mut temp = Vec::new();

    for i in 0..(vec.len() - len) {
        temp.push(vec[i]);
    }

    let index = bool2_int(temp);
    temp = Vec::new();

    for i in 0..len {
        temp.push(vec[vec.len() - len + i]);
    }

    (index, temp)
}

fn match_table(vec_2d: Vec<Vec<bool>>, len: usize, init: bool) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for i in 0..len {
        result.push(vec![init]);
        for _ in 1..pow2(vec_2d[0].len() - len) {
            result[i].push(init);
        }
    }

    for b in vec_2d {
        let (index, vec) = match_line(b, len);
        for i in 0..vec.len() {
            result[i][index] = vec[i];
        }
    }

    result
}

// ---------------------------------------------------------------------- Tests ----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_match_line() {
        assert_eq!(
            match_line(vec![true, false, true, false], 2),
            (2, vec![true, false])
        );
        assert_eq!(
            match_line(vec![true, false, true, false], 1),
            (5, vec![false])
        );

        assert_eq!(
            match_line(vec![false, false, true, false], 1),
            (1, vec![false])
        );
    }

    #[test]
    fn test_match_table() {
        assert_eq!(
            match_table(
                vec![
                    vec![true, true, true],
                    vec![false, false, false],
                    vec![false, true, false],
                    vec![true, false, false],
                ],
                1,
                true
            ),
            vec![vec![false, false, false, true]]
        );
        assert_eq!(
            match_table(vec![vec![true, true, true],], 1, false),
            vec![vec![false, false, false, true]]
        );
    }

    #[test]
    fn test_split_rows() {
        assert_eq!(
            split_rows(
                str2_bool(
                    "111
                    000
                    010
                    100"
                ),
                3
            ),
            vec![
                vec![true, true, true],
                vec![false, false, false],
                vec![false, true, false],
                vec![true, false, false],
            ]
        );
    }

    #[test]
    fn test_pow2() {
        assert_eq!(pow2(0), 1);
        assert_eq!(pow2(1), 2);
        assert_eq!(pow2(2), 4);
        assert_eq!(pow2(3), 8);
        assert_eq!(pow2(4), 16);
    }

    #[test]
    fn test_str2_bool() {
        assert_eq!(
            str2_bool("100011"),
            vec![true, false, false, false, true, true]
        );
    }

    #[test]
    fn test_bool2_int() {
        assert_eq!(bool2_int(vec![false, false, false]), 0);
        assert_eq!(bool2_int(vec![false, false, true]), 1);
        assert_eq!(bool2_int(vec![false, true, true]), 3);
        assert_eq!(bool2_int(vec![true, false, false]), 4);
    }
}
