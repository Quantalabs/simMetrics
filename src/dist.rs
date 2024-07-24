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
    let (m, n) = (s.len(), t.len());

    // Ensure the shorter string is 's' for memory efficiency
    if m < n {
        return _l_distance(t, s);
    }

    let mut prev_row = (0..=n).collect::<Vec<usize>>();
    let mut curr_row = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                curr_row[j] = prev_row[j - 1];
            } else {
                curr_row[j] = 1 + prev_row[j].min(curr_row[j - 1]).min(prev_row[j - 1]);
            }
        }
        prev_row.copy_from_slice(&curr_row);
    }

    prev_row[n]
}

pub fn l_distance(s: &str, t: &str) -> usize {
    let _s = s.chars().collect::<Vec<char>>();
    let _t = t.chars().collect::<Vec<char>>();

    _l_distance(&_s, &_t)
}
