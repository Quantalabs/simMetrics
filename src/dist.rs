#![allow(non_upper_case_globals)]

fn nonzero_jaro_sim(a: &str, b: &str) -> f64 {
    0.0
}

/// Return longer length of two strings
const longer: fn(&str, &str) -> isize = |a: &str, b: &str|
    std::cmp::max(a.len() as isize, b.len() as isize);

/// Return tuple of longer string then shorter string
const order: for<'a> fn(&'a str, &'a str) -> (&'a str, &'a str) = |a: &str, b: &str|
    if a.len() > b.len() { (a, b) } else { (b, a) };

/// Check whether `a` is the `i`th element of string `b`
const is: fn(char, &str, isize) -> bool =
    |a: char, b: &str, i: isize|
        if i < 0 { false } else { a == b.chars().nth(i as usize).unwrap() };

/// Check whether the `i`th character of some string, `a`, has a match in string `b`
/// within a radius of `r` characters. If a match exists, return the index of the match.
/// Otherwise, return `None`.
const matches: fn(char, &str, isize, isize) -> Option<isize> =
    |a: char, b: &str, i: isize, r: isize|
        (
            std::cmp::max(i - r, 0)
            ..=std::cmp::min(i + r, b.len() as isize)
        ).find(|&j| is(a, b, j));

/// Equivalent of `matches` but accepts an additional argument of previous matches `acc` and an
/// offset for keeping track of indices for recursive calls (this should be set to 0 initially).
/// This will only return a new match if it is not already in `acc`.
pub fn unique_matches(
    a: char, b: &str,
    i: isize, r: isize,
    acc: &Vec<isize>, offset: isize
) -> Option<isize> {
    match matches(a, b, i, r) {
        Some(j) => {
            if acc.contains(&(j + offset)) {
                let size = (b.len() as isize) - j - 1;
                if size <= 0 { None }
                else {
                    let cut = b.chars().rev().take(size as usize)
                        .collect::<String>().chars().rev()
                        .collect::<String>();
                    unique_matches(a, &cut, i - j - 1, r, acc, j + 1)
                }
            }
            else { Some(j) }
        },
        None => None,
    }
}

/// Calculate Jaro matching character radius
const radius: fn(&str, &str) -> isize = |a: &str, b: &str| (longer(a, b) / 2) - 1;

/// Number of characters considered "matching" by Jaro-Winkler
/// 
/// # Examples
/// 
/// ```
/// use similarity_metrics::dist::matching;
/// assert_eq!(matching("hello", "hello world"), 5);
/// assert_eq!(matching("FAREMVIEL", "FARMVILLE"), 8);
/// ```
pub fn matching(a: &str, b: &str) -> isize {
    let r = radius(a, b);
    let (long, short) = order(a, b);
    let mut acc: Vec<isize> = Vec::new();
    
    for i in 0..short.len() {
        if let Some(j) = unique_matches(
            short.chars().nth(i).unwrap(),
            long, i as isize, r,
            &acc, 0
        ) {
            acc.push(j);
        }
    }

    acc.len() as isize
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
pub fn jaro_winkler_ext(a: &str, b: &str, p: Option<f64>, l: Option<isize>) -> f64 {
    0.0
}
