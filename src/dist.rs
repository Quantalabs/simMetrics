fn lcs_helper(
    x: &[char],
    y: &[char],
    i: usize,
    j: usize,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if i == 0 || j == 0 {
        return 0;
    }

    if let Some(value) = memo[i][j] {
        return value;
    }

    if x[i - 1] == y[j - 1] {
        let result = 1 + lcs_helper(x, y, i - 1, j - 1, memo);
        memo[i][j] = Some(result);
        result
    } else {
        let result = usize::max(
            lcs_helper(x, y, i - 1, j, memo),
            lcs_helper(x, y, i, j - 1, memo),
        );
        memo[i][j] = Some(result);
        result
    }
}

fn _lcs(x: &[char], y: &[char]) -> usize {
    let m = x.len();
    let n = y.len();

    let mut memo = vec![vec![None; n + 1]; m + 1];

    lcs_helper(x, y, m, n, &mut memo)
}

pub fn lcs(x: &str, y: &str) -> usize {
    let _x = x.chars().collect::<Vec<char>>();
    let _y = y.chars().collect::<Vec<char>>();

    _lcs(&_x, &_y)
}

pub fn _l_distance(s: &[char], t: &[char]) -> usize {
    match (s.is_empty(), t.is_empty()) {
        (true, true) => 0,
        (true, false) => t.len(),
        (false, true) => s.len(),
        (false, false) => {
            let cost = if s[0] == t[0] { 0 } else { 1 };
            let insertion = _l_distance(s, &t[1..]) + 1;
            let deletion = _l_distance(&s[1..], t) + 1;
            let substitution = _l_distance(&s[1..], &t[1..]) + cost;
            *[insertion, deletion, substitution].iter().min().unwrap()
        }
    }
}

pub fn l_distance(s: &str, t: &str) -> usize {
    let _s = s.chars().collect::<Vec<char>>();
    let _t = t.chars().collect::<Vec<char>>();

    _l_distance(&_s, &_t)
}
