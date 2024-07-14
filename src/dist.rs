#![allow(non_upper_case_globals)]

fn nonzero_jaro_sim(a: &str, b: &str) -> f64 {
    0.0
}

// Check whether `a` is the `i`th element of string `b`
const is_in: fn(char, &str, usize) -> bool =
    |a: char, b: &str, i: usize| a == b.chars().nth(i).unwrap();

/// Check whether the `i`th character of some string, `a`, has a match in string `b`
/// within a radius of `r` characters. If a match exists, return the index of the match.
/// Otherwise, return `None`.
const matches: fn(char, &str, usize, usize) -> Option<usize> =
    |a: char, b: &str, i: usize, r: usize|
        ((i - r)..=(std::cmp::min(i + r, b.len()))).find(|&j| is_in(a, b, j));

/// Number of characters considered "matching" by Jaro-Winkler
pub fn matching(a: &str, b: &str) -> usize {
    0
}

/// Jaro distance, in [0, 1]
/// 
/// Given two strings `a` and `b`, calculate the Jaro distance between them.
/// 
/// 0 indicates that `a` and `b` are exactly the same.
/// 1 indicates that there is no similarity between `a` and `b`.
pub fn jaro(a: &str, b: &str) -> f64 {
    if matching(a, b) == 0 { 0.0 } else { nonzero_jaro_sim(a, b) }
}

/// Jaro–Winkler distance, in [0, 1]  
/// See: https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
/// 
/// Given two strings `a` and `b`, calculate the Jaro–Winkler distance between them.
/// The lower this distance is, the more similar `a` and `b` are.
/// `p` is the weight of the prefix, and defaults to 0.1.
/// Values of `p` higher than 0.25 will be capped at 0.25.
/// 
/// 0 indicates that `a` and `b` are exactly the same.  
/// 1 indicates that there is no similarity between `a` and `b`. 
pub fn jaro_winkler(a: &str, b: &str, p: Option<f64>) -> f64 {
    let p = f64::min(p.unwrap_or(0.1), 0.25);
    0.0
}

/// Jaro–Winkler distance with custom prefix length, in [0, 1]
/// 
/// Given two strings `a` and `b`, calculate the Jaro–Winkler distance between them,
/// considering prefixes up to length `l`.
/// 
/// `p` is the weight of the prefix, and defaults to 0.1.
/// `l` is the length of the common prefix, and defaults to the size of the shorter string.
/// 
/// 0 indicates that `a` and `b` are exactly the same.
/// 1 indicates that there is no similarity between `a` and `b`.
pub fn jaro_winkler_ext(a: &str, b: &str, p: Option<f64>, l: Option<usize>) -> f64 {
    0.0
}
