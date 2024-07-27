#![allow(non_upper_case_globals)]

pub type Pointer = Option<isize>;
pub type Locations = Vec<isize>;

#[derive(Debug)]
struct Ordered {
    long: String,
    short: String
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
const order: fn(String, String) -> Ordered = |a: String, b: String|
    if a.len() > b.len() { Ordered { long: a, short: b } }
    else { Ordered { long: b, short: a } };

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
/// let acc: Locations = vec![0, 2];
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
/// let empty: Locations = vec![];
/// assert_eq!(append(empty, 0), [0]);
/// 
/// let singleton: Locations = vec![0];
/// assert_eq!(append(singleton, 2), [0, 2]);
/// 
/// let acc: Locations = vec![0, 2];
/// assert_eq!(append(acc, 3), [0, 2, 3]);
/// 
/// let acc2: Locations = vec![0, 2, 3];
/// assert_eq!(append(acc2, 6), [0, 2, 3, 6]);
/// ```
pub const append: fn(Locations, isize) -> Locations =
    |acc: Locations, i: isize| acc.iter().cloned().chain(std::iter::once(i)).collect();

/// Number of matched indices in permutation,
/// as compared to a monotonically increasing array of list indices;
/// e.g. [0], [0, 1], [0, 1, 2], ...
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::matched;
/// 
/// let acc: Locations = vec![0, 2];
/// assert_eq!(matched(&acc), 1);
/// 
/// let acc2: Locations = vec![2, 2, 2, 2];
/// assert_eq!(matched(&acc2), 1);
/// 
/// let acc3: Locations = vec![0, 1, 2, 3];
/// assert_eq!(matched(&acc3), 4);
/// ```
pub const matched: fn(&Locations) -> isize = |permutation: &Locations|
    permutation.iter().enumerate().fold(0,
        |acc, (i, &j)|
            acc + if (i as isize) == j { 1 } else { 0 }
    );

/// Number of unmatched indices in permutation
/// as compared to a monotonically increasing array of list indices;
/// e.g. [0], [0, 1], [0, 1, 2], ...
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::unmatched;
/// 
/// let acc: Locations = vec![0, 2];
/// assert_eq!(unmatched(&acc), 1);
/// 
/// let acc2: Locations = vec![2, 2, 2, 2];
/// assert_eq!(unmatched(&acc2), 3);
/// 
/// let acc3: Locations = vec![0, 1, 2, 3];
/// assert_eq!(unmatched(&acc3), 0);
/// ```
pub const unmatched: fn(&Locations) -> isize = |permutation: &Locations|
    permutation.len() as isize - matched(permutation);

/// Generate all possible shifts of a permutation,
/// keeping overflow (which won't affect matching counts) and padding the beginning
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::shifts;
/// 
/// let acc: Locations = vec![0, 2];
/// let shifted: Vec<Locations> = vec![vec![0, 2], vec![0, 0, 2]];
/// assert_eq!(shifts(acc), shifted);
/// 
/// let acc2: Locations = vec![1, 2, 3, 4];
/// let shifted2: Vec<Locations> = vec![
///     vec![1, 2, 3, 4],
///     vec![0, 1, 2, 3, 4],
///     vec![0, 0, 1, 2, 3, 4],
///     vec![0, 0, 0, 1, 2, 3, 4],
/// ];
/// assert_eq!(shifts(acc2), shifted2);
/// 
/// let empty: Locations = vec![];
/// let empty2: Vec<Locations> = vec![];
/// assert_eq!(shifts(empty), empty2);
/// 
/// let singleton: Locations = vec![0];
/// assert_eq!(shifts(singleton.clone()), vec![singleton]);
/// ```
pub const shifts: fn(Locations) -> Vec<Locations> =
    |permutation: Locations|
        (0..permutation.len() as isize).map(
            |i| [
                std::iter::repeat(0).take(i as usize).collect(),
                permutation.clone()
            ].concat()
        ).collect();

/// Generate all possible reverse shifts of a permutation,
/// discarding overflow to prevent matching counts from being affected and padding the end
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::revShifts;
/// 
/// let acc: Locations = vec![0, 2];
/// let shifted: Vec<Locations> = vec![vec![2, 0]];
/// assert_eq!(revShifts(acc), shifted);
/// 
/// let acc2: Locations = vec![1, 2, 3, 4];
/// let shifted2: Vec<Locations> = vec![
///     vec![2, 3, 4, 0],
///     vec![3, 4, 0, 0],
///     vec![4, 0, 0, 0]
/// ];
/// assert_eq!(revShifts(acc2), shifted2);
/// 
/// let empty: Locations = vec![];
/// let empty2: Vec<Locations> = vec![];
/// assert_eq!(revShifts(empty), empty2.clone());
/// 
/// let singleton: Locations = vec![0];
/// assert_eq!(revShifts(singleton.clone()), empty2);
/// ```
pub const revShifts: fn(Locations) -> Vec<Locations> =
    |permutation: Locations|
        (1..permutation.len() as isize).map(
            |i| [
                permutation.iter().skip(i as usize).cloned().collect::<Locations>(),
                std::iter::repeat(0).take(i as usize).collect()
            ].concat()
        ).collect();

/// Halve an integer and round up if odd
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::roundHalf;
/// 
/// assert_eq!(roundHalf(0), 0);
/// assert_eq!(roundHalf(1), 1);
/// assert_eq!(roundHalf(2), 1);
/// assert_eq!(roundHalf(3), 2);
/// assert_eq!(roundHalf(4), 2);
/// ```
pub const roundHalf: fn(isize) -> isize = |x: isize| (x / 2) + (x % 2);

/// Number of transpositions in permutation  
/// 
/// Note that transpositions are not calculated in the standard way;
/// instead of calculating the number of swaps required to arrange two strings
/// in the same order, this calculates the number of characters that are
/// "out of place" and then divides that by two, rounding up.
/// This results in faster transposition estimates,
/// though in rare cases these estimates can differ from calculations
/// using the traditional method of counting swaps.
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::transpositions;
/// 
/// let acc: Locations = vec![0, 2, 1, 3];
/// assert_eq!(transpositions(acc), 1);
/// 
/// let acc2: Locations = vec![0, 1, 2, 3];
/// assert_eq!(transpositions(acc2), 0);
/// 
/// let acc3: Locations = vec![1, 2, 3, 4, 0];
/// assert_eq!(transpositions(acc3), 1);
/// ```
pub const transpositions: fn(Locations) -> isize = |permutation: Locations|
    roundHalf([
        shifts(permutation.clone()),
        revShifts(permutation)
    ].concat().iter()
     .map(unmatched).min()
     .unwrap_or(0));

/// Assign each item in the locations list a "rank"  
/// See examples for details
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::Locations;
/// use similarity_metrics::dist::sequence;
/// 
/// let acc: Locations = vec![0, 2, 1, 5];
/// let sequenced: Locations = vec![0, 2, 1, 3];
/// assert_eq!(sequence(acc), sequenced);
/// 
/// let acc2: Locations = vec![9, 8, 5, 0];
/// let sequenced2: Locations = vec![3, 2, 1, 0];
/// assert_eq!(sequence(acc2), sequenced2);
/// ```
pub const sequence: fn(Locations) -> Locations = |permutation: Locations|
    permutation.iter().map(
        |k: &isize| permutation.iter().filter(|&i| i < k).count() as isize
    ).collect();

const matching_ordered: fn(Ordered, isize) -> Locations =
    |pair: Ordered, r: isize|
        (0..pair.short.len()).fold(
            Vec::new(),
            |acc, i|
                if let Some(j) = unique_matches(
                    pair.short.chars().nth(i).unwrap(),
                    pair.long.as_str(), i as isize, r,
                    &acc, 0
                ) { append(acc, j) } else { acc }
        );

const gen_jaro_metrics: fn(Locations) -> (isize, isize) = |permutation: Locations|
    (permutation.len() as isize, transpositions(sequence(permutation)));

/// Number of characters considered "matching" by Jaro-Winkler
/// and the number of transpositions required to match those characters  
/// 
/// Transpositions are calculated according to a custom algorithm
/// (see `similarity_metrics::dist::transpositions` for more details)
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::matching;
/// assert_eq!(matching("hello".to_string(), "hello world".to_string()), (5, 0));
/// assert_eq!(matching("---amyez---".to_string(), "---zayem---".to_string()), (11, 2));
/// assert_eq!(matching("---zayem---".to_string(), "---amyez---".to_string()), (11, 2));
/// assert_eq!(matching("FAREMVIEL".to_string(), "FARMVILLE".to_string()), (8, 1));
/// assert_eq!(matching("winkler".to_string(), "welfare".to_string()), (4, 1));
/// assert_eq!(matching("DWAYNE".to_string(), "DUANE".to_string()), (4, 0));
/// assert_eq!(matching("DUANE".to_string(), "DWAYNE".to_string()), (4, 0));
/// assert_eq!(matching("martha".to_string(), "marhta".to_string()), (6, 1));
/// assert_eq!(matching("DIXON".to_string(), "DIRKSONX".to_string()), (4, 0));
/// assert_eq!(matching("JeLlYfIsH".to_string(), "SMeLlYfIsH".to_string()), (8, 0));
/// assert_eq!(matching("UPPERCASE".to_string(), "lowercase".to_string()), (0, 0));
/// assert_eq!(matching("UPPERCASE".to_string(), "lowerCASE".to_string()), (4, 0));
/// assert_eq!(matching("!@#ABCDE$%^".to_string(), "$%^EABCD!@#".to_string()), (5, 1));
/// assert_eq!(matching("$%^EABCD!@#".to_string(), "!@#ABCDE$%^".to_string()), (5, 1));
/// assert_eq!(matching("".to_string(), "".to_string()), (0, 0));
/// assert_eq!(matching("a".to_string(), "".to_string()), (0, 0));
/// assert_eq!(matching("a".to_string(), "ab".to_string()), (1, 0));
/// ```
pub const matching: fn(String, String) -> (isize, isize) = |a: String, b: String|
    gen_jaro_metrics(matching_ordered(order(a.clone(), b.clone()), radius(a.as_str(), b.as_str())));

/// Jaro similarity, in [0, 1]
/// 
/// Given a number of matching characters `n_matching` and
/// a number of transpositions `n_transpositions`,
/// calculate the Jaro distance between two strings `a` and `b`.
/// Helper function for `similarity_metrics::dist::jaro`.
const nonzero_jaro_sim: fn(&str, &str, isize, isize) -> f64 =
    |a: &str, b: &str, n_matching: isize, n_transpositions: isize|
        1.0 / 3.0 * (
            n_matching as f64 / a.len() as f64
            + n_matching as f64 / b.len() as f64
            + (n_matching - n_transpositions) as f64 / n_matching as f64
        );

/// Choose correct Jaro helper function
const jaro_sim: fn(&str, &str, (isize, isize)) -> f64 =
    |a: &str, b: &str, metrics: (isize, isize)|
        if metrics.0 == 0 { 0.0 } else { nonzero_jaro_sim(a, b, metrics.0, metrics.1) };

/// Jaro distance, in [0, 1]
/// 
/// Given two strings `a` and `b`, calculate the Jaro distance between them.
/// 
/// 0 indicates that `a` and `b` are exactly the same.
/// 1 indicates that there is no similarity between `a` and `b`.
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::jaro;
/// const epsilon: f64 = 1e-6;
/// const around: fn(f64, f64) -> bool = |a: f64, b: f64| (a - b).abs() <= epsilon;
/// const frac: fn(isize, isize) -> f64 = |a: isize, b: isize| (a as f64) / (b as f64);
/// const zero: f64 = 0.0;
/// assert!(around(jaro("hello".to_string(), "hello world".to_string()), frac(9, 11)));
/// assert!(around(jaro("---amyez---".to_string(), "---zayem---".to_string()), frac(31, 33)));
/// assert!(around(jaro("---zayem---".to_string(), "---amyez---".to_string()), frac(31, 33)));
/// assert!(around(jaro("FAREMVIEL".to_string(), "FARMVILLE".to_string()), frac(191, 216)));
/// assert!(around(jaro("winkler".to_string(), "welfare".to_string()), frac(53, 84)));
/// assert!(around(jaro("DWAYNE".to_string(), "DUANE".to_string()), frac(37, 45)));
/// assert!(around(jaro("DUANE".to_string(), "DWAYNE".to_string()), frac(37, 45)));
/// assert!(around(jaro("martha".to_string(), "marhta".to_string()), frac(17, 18)));
/// assert!(around(jaro("DIXON".to_string(), "DIRKSONX".to_string()), frac(23, 30)));
/// assert!(around(jaro("JeLlYfIsH".to_string(), "SMeLlYfIsH".to_string()), frac(121, 135)));
/// assert!(around(jaro("UPPERCASE".to_string(), "lowercase".to_string()), zero));
/// assert!(around(jaro("UPPERCASE".to_string(), "lowerCASE".to_string()), frac(17, 27)));
/// assert!(around(jaro("!@#ABCDE$%^".to_string(), "$%^EABCD!@#".to_string()), frac(94, 165)));
/// assert!(around(jaro("$%^EABCD!@#".to_string(), "!@#ABCDE$%^".to_string()), frac(94, 165)));
/// assert!(around(jaro("".to_string(), "".to_string()), zero));
/// assert!(around(jaro("a".to_string(), "".to_string()), zero));
/// assert!(around(jaro("a".to_string(), "ab".to_string()), frac(5, 6)));
/// ```
pub const jaro: fn(String, String) -> f64 =
    |a: String, b|
        jaro_sim(a.clone().as_str(), b.clone().as_str(), matching(a, b));

/// Length of longest common prefix between two strings
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::len_common_prefix;
/// assert_eq!(len_common_prefix("hello", "hello world"), 5);
/// assert_eq!(len_common_prefix("hello", "world"), 0);
/// assert_eq!(len_common_prefix("hello", "hell"), 4);
/// assert_eq!(len_common_prefix("hello", "he"), 2);
/// ```
pub const len_common_prefix: fn(&str, &str) -> isize = |a: &str, b: &str|
    a.chars().zip(b.chars()).take_while(|(c1, c2)| c1 == c2).count() as isize;

/// Jaro–Winkler distance, in [0, 1]
/// 
/// Given a Jaro distance `dist` and a prefix length and weight (`l` and `p`, respectively),
/// calculate the Jaro–Winkler distance.
/// Helper function for `similarity_metrics::dist::jaro_winkler`.
const jaro_winkler_sim: fn(f64, f64, isize) -> f64 =
    |dist: f64, p: f64, l: isize| dist + (l as f64) * p * (1.0 - dist);

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
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::jaro_winkler;
/// const epsilon: f64 = 1e-6;
/// const around: fn(f64, f64) -> bool = |a: f64, b: f64| (a - b).abs() <= epsilon;
/// const frac: fn(isize, isize) -> f64 = |a: isize, b: isize| (a as f64) / (b as f64);
/// const zero: f64 = 0.0;
/// assert!(around(
///     jaro_winkler("hello".to_string(), "hello world".to_string(), None), frac(49, 55)
/// ));
/// assert!(around(
///     jaro_winkler(
///         "---amyez---".to_string(), "---zayem---".to_string(), Some(0.2)
///     ), frac(161, 165)
/// ));
/// assert!(around(
///     jaro_winkler(
///         "---zayem---".to_string(), "---amyez---".to_string(), Some(0.2)
///     ), frac(161, 165)
/// ));
/// assert!(around(
///     jaro_winkler("FAREMVIEL".to_string(), "FARMVILLE".to_string(), None), frac(397, 432)
/// ));
/// assert!(around(
///     jaro_winkler("winkler".to_string(), "welfare".to_string(), Some(0.4)), frac(81, 112)
/// ));
/// assert!(around(
///     jaro_winkler("DWAYNE".to_string(), "DUANE".to_string(), None), frac(21, 25)
/// ));
/// assert!(around(
///     jaro_winkler("DUANE".to_string(), "DWAYNE".to_string(), None), frac(21, 25)
/// ));
/// assert!(around(
///     jaro_winkler("martha".to_string(), "marhta".to_string(), Some(0.05)), frac(343, 360)
/// ));
/// assert!(around(
///     jaro_winkler("DIXON".to_string(), "DIRKSONX".to_string(), None), frac(61, 75)
/// ));
/// assert!(around(
///     jaro_winkler("JeLlYfIsH".to_string(), "SMeLlYfIsH".to_string(), Some(0.5)), frac(121, 135)
/// ));
/// assert!(around(
///     jaro_winkler("UPPERCASE".to_string(), "lowercase".to_string(), None), zero
/// ));
/// assert!(around(
///     jaro_winkler("UPPERCASE".to_string(), "lowerCASE".to_string(), None), frac(17, 27)
/// ));
/// assert!(around(
///     jaro_winkler("!@#ABCDE$%^".to_string(), "$%^EABCD!@#".to_string(), None), frac(94, 165)
/// ));
/// assert!(around(
///     jaro_winkler("$%^EABCD!@#".to_string(), "!@#ABCDE$%^".to_string(), None), frac(94, 165)
/// ));
/// assert!(around(
///     jaro_winkler("".to_string(), "".to_string(), None), zero
/// ));
/// assert!(around(
///     jaro_winkler("a".to_string(), "".to_string(), None), zero
/// ));
/// assert!(around(
///     jaro_winkler("a".to_string(), "ab".to_string(), Some(0.25)), frac(7, 8)
/// ));
/// assert!(around(
///     jaro_winkler("ab".to_string(), "a".to_string(), Some(zero)), frac(5, 6)
/// ))
/// ```
pub const jaro_winkler: fn(String, String, Option<f64>) -> f64 =
    |a: String, b: String, p: Option<f64>|
        jaro_winkler_sim(
            jaro(a.clone(), b.clone()),
            f64::min(p.unwrap_or(0.1), 0.25),
            std::cmp::min(len_common_prefix(a.as_str(), b.as_str()), 4)
        );

/// Jaro–Winkler distance with custom prefix length, in [0, 1]
/// 
/// Given a Jaro distance `dist` and a prefix length and weight (`l` and `p`, respectively),
/// calculate the Jaro–Winkler distance, capping `p` and `l` if necessary.
/// To cap `p` and `l`, this also takes in the maximum possible prefix length `max_l` specified.
/// Helper function for `similarity_metrics::dist::jaro_winkler_ext`.
const jaro_winkler_ext_sim: fn(f64, f64, isize, isize) -> f64 =
    |dist: f64, p: f64, l: isize, max_l: isize|
        jaro_winkler_sim(
            dist,
            if (max_l as f64) * p <= 1.0 { p } else { 1.0 / max_l as f64 },
            std::cmp::min(l, max_l)
        );

/// Jaro–Winkler distance with custom prefix length, in [0, 1]
/// 
/// Given two strings `a` and `b`, calculate the Jaro–Winkler distance between them,
/// considering prefixes up to length `l`.
/// 
/// `p` is the weight of the prefix, and defaults to 0.1 (or highest possible for prefix length).
/// `l` is the length of the common prefix, and defaults to the size of the shorter string.
/// Values of `p` that, combined with the value of `l`, could result in a similarity
/// greater than 1 are capped.
/// 
/// 0 indicates that `a` and `b` are exactly the same.
/// 1 indicates that there is no similarity between `a` and `b`.
/// 
/// ## Examples
/// 
/// ```
/// use similarity_metrics::dist::jaro_winkler_ext;
/// const epsilon: f64 = 1e-6;
/// const around: fn(f64, f64) -> bool = |a: f64, b: f64| (a - b).abs() <= epsilon;
/// const frac: fn(isize, isize) -> f64 = |a: isize, b: isize| (a as f64) / (b as f64);
/// const one: f64 = 1.0;
/// assert!(around(
///     jaro_winkler_ext("hello".to_string(), "hello world".to_string(), None, None), frac(10, 11)
/// ));
/// assert!(around(
///     jaro_winkler_ext(
///         "---amyez---".to_string(), "---zayem---".to_string(), Some(0.5), Some(2)
///     ), one
/// ));
/// assert!(around(
///     jaro_winkler_ext(
///         "---zayem---".to_string(), "---amyez---".to_string(), Some(0.2), Some(3)
///     ), frac(161, 165)
/// ));
/// assert!(around(
///     jaro_winkler_ext("FAREMVIEL".to_string(), "FARMVILLE".to_string(), None, None), frac(397, 432)
/// ));
/// assert!(around(
///     jaro_winkler_ext("winkler".to_string(), "welfare".to_string(), Some(0.25), None), frac(67, 98)
/// ));
/// ```
pub const jaro_winkler_ext: fn(String, String, Option<f64>, Option<isize>) -> f64 =
    |a: String, b: String, p: Option<f64>, l: Option<isize>|
        jaro_winkler_ext_sim(
            jaro(a.clone(), b.clone()),
            p.unwrap_or(0.1),
            len_common_prefix(a.as_str(), b.as_str()),
            l.unwrap_or(std::cmp::min(a.len() as isize, b.len() as isize))
        );
