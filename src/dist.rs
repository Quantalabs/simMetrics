#![allow(non_upper_case_globals)]

pub type Pointer = Option<isize>;
pub type Locations = Vec<isize>;

#[derive(Debug)]
struct Ordered {
    long: &'static str,
    short: &'static str
}

fn nonzero_jaro_sim(a: &str, b: &str) -> f64 {
    0.0
}

/// Return longer length of two strings
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::longer;
/// assert_eq!(longer("hello", "hello world"), 11);
/// assert_eq!(longer("hello", "hello"), 5);
/// assert_eq!(longer("CAPS", "lower"), 5);
/// assert_eq!(longer("", "xyz"), 3);
/// assert_eq!(longer("abcdefgh", ""), 8);
/// assert_eq!(longer("x", ""), 1);
/// assert_eq!(longer("", "x"), 1);
/// assert_eq!(longer("", ""), 0);
/// ```
pub const longer: fn(&str, &str) -> isize = |a: &str, b: &str|
    std::cmp::max(a.len() as isize, b.len() as isize);

/// Return ordered pair of strings
const order: fn(&'static str, &'static str) -> Ordered = |a: &str, b: &str|
    if a.len() > b.len() { Ordered { long: a, short: b } } else { Ordered { long: b, short: a } };

/// Check whether `a` is the `i`th element of string `b`
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::is;
/// assert_eq!(is('h', "hello", 0), true);
/// assert_eq!(is('e', "hello", 0), false);
/// assert_eq!(is('e', "hello", 2), false);
/// assert_eq!(is('l', "hello", 3), true);
/// assert_eq!(is('l', "hello", 4), false);
/// assert_eq!(is('o', "hello", 4), true);
/// assert_eq!(is('o', "hello", 6), false);
/// assert_eq!(is('x', "hello", 7), false);
/// assert_eq!(is('x', "hello", 0), false);
/// ```
pub const is: fn(char, &str, isize) -> bool =
    |a: char, b: &str, i: isize|
        if i < 0 { false } else if let Some(c) = b.chars().nth(i as usize) { a == c }
        else { false };

/// Check whether the `i`th character of some string, `a`, has a match in string `b`
/// within a radius of `r` characters. If a match exists, return the index of the match.
/// Otherwise, return `None`.
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::matches;
/// assert_eq!(matches('h', "hello", 0, 2), Some(0));
/// assert_eq!(matches('o', "hello", 0, 2), None);
/// assert_eq!(matches('e', "hello", 2, 1), Some(1));
/// assert_eq!(matches('x', "hello", 0, 2), None);
/// ```
pub const matches: fn(char, &str, isize, isize) -> Pointer =
    |a: char, b: &str, i: isize, r: isize|
        (
            std::cmp::max(i - r, 0)
            ..=std::cmp::min(i + r, b.len() as isize)
        ).find(|&j| is(a, b, j));

/// Get the last `i` characters of string `a`
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::last;
/// assert_eq!(last("hello", 5), "hello");
/// assert_eq!(last("hello", 3), "llo");
/// assert_eq!(last("hello", 0), "");
/// ```
pub const last: fn(&str, usize) -> &str = |a: &str, i: usize| &a[(a.len() - i)..];

/// Equivalent of `matches` but accepts an additional argument of previous matches `acc` and an
/// offset for keeping track of indices for recursive calls (this should be set to 0 initially).
/// This will only return a new match if it is not already in `acc`.
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::unique_matches;
/// let acc: Locations = [0, 2].to_vec();
/// assert_eq!(unique_matches('h', "hello", 0, 2, &acc, 0), None);
/// assert_eq!(unique_matches('o', "hello", 0, 2, &acc, 0), None);
/// assert_eq!(unique_matches('l', "hello", 2, 1, &acc, 0), Some(3));
/// assert_eq!(unique_matches('l', "hello", 2, 0, &acc, 0), None);
/// assert_eq!(unique_matches('m', "mammal", 2, 3, &acc, 0), Some(3));
/// ```
pub const unique_matches: fn(char, &str, isize, isize, &Locations, isize) -> Pointer =
    | a: char, b: &str, i: isize, r: isize, acc: &Locations, offset: isize |
        match matches(a, b, i, r) {
            Some(j) => {
                if acc.contains(&(j + offset)) {
                    let size = (b.len() as isize) - j - 1;
                    if size <= 0 { None } else {
                        unique_matches(
                            a, last(b, size as usize),
                            i - j - 1, r,
                            acc, j + offset + 1
                        )
                    }
                } else { Some(j + offset) }
            },
            None => None,
        };

/// Calculate Jaro matching character radius
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::radius;
/// assert_eq!(radius("hello", "hello world"), 4);
/// assert_eq!(radius("FAREMVIEL", "FARMVILLE"), 3);
/// assert_eq!(radius("winkler", "welfare"), 2);
/// assert_eq!(radius("DWAYNE", "DUANE"), 2);
/// assert_eq!(radius("five", "four"), 1);
/// assert_eq!(radius("hi", "low"), 0);
/// assert_eq!(radius("martha", "marhta"), 2);
/// assert_eq!(radius("DIXON", "DIRKSONX"), 3);
/// assert_eq!(radius("JeLlYfIsH", "SMeLlYfIsH"), 4);
/// assert_eq!(radius("UPPERCASE", "lowercase"), 3);
/// assert_eq!(radius("UPPERCASE", "lowerCASE"), 3);
/// ```
pub const radius: fn(&str, &str) -> isize = |a: &str, b: &str|
    std::cmp::max((longer(a, b) / 2) - 1, 0);

/// Append `i` to `Locations`
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::append;
/// 
/// let empty: Locations = [].to_vec();
/// assert_eq!(append(empty, 0), [0]);
/// 
/// let singleton: Locations = [0].to_vec();
/// assert_eq!(append(singleton, 2), [0, 2]);
/// 
/// let acc: Locations = [0, 2].to_vec();
/// assert_eq!(append(acc, 3), [0, 2, 3]);
/// 
/// let acc2: Locations = [0, 2, 3].to_vec();
/// assert_eq!(append(acc2, 6), [0, 2, 3, 6]);
/// ```
pub const append: fn(Locations, isize) -> Locations =
    |acc: Locations, i: isize| acc.iter().cloned().chain(std::iter::once(i)).collect();

const matching_ordered: fn(Ordered, isize) -> isize =
    |pair: Ordered, r: isize|
        (0..pair.short.len()).fold(
            Vec::new(),
            |acc, i|
                if let Some(j) = unique_matches(
                    pair.short.chars().nth(i).unwrap(),
                    pair.long, i as isize, r,
                    &acc, 0
                ) { append(acc, j) } else { acc }
        ).len() as isize;

/// Number of characters considered "matching" by Jaro-Winkler
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::matching;
/// assert_eq!(matching("hello", "hello world"), 5);
/// assert_eq!(matching("FAREMVIEL", "FARMVILLE"), 8);
/// assert_eq!(matching("winkler", "welfare"), 4);
/// assert_eq!(matching("DWAYNE", "DUANE"), 4);
/// assert_eq!(matching("martha", "marhta"), 6);
/// assert_eq!(matching("DIXON", "DIRKSONX"), 4);
/// assert_eq!(matching("JeLlYfIsH", "SMeLlYfIsH"), 8);
/// assert_eq!(matching("UPPERCASE", "lowercase"), 0);
/// assert_eq!(matching("UPPERCASE", "lowerCASE"), 4);
/// ```
pub const matching: fn(&'static str, &'static str) -> isize = |a: &str, b: &str|
    matching_ordered(order(a, b), radius(a, b));

/// Jaro distance, in [0, 1]
/// 
/// Given two strings `a` and `b`, calculate the Jaro distance between them.
/// 
/// 0 indicates that `a` and `b` are exactly the same.
/// 1 indicates that there is no similarity between `a` and `b`.
pub fn jaro(a: &'static str, b: &'static str) -> f64 {
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
