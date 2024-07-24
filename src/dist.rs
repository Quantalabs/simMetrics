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

    // Create a memoization table
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill the dp table
    for i in 1..=m {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[m][n]
}

pub fn l_distance(s: &str, t: &str) -> usize {
    let _s = s.chars().collect::<Vec<char>>();
    let _t = t.chars().collect::<Vec<char>>();

    _l_distance(&_s, &_t)
}
