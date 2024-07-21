fn abc(f1: &[u32], f2: &[u32]) -> (usize, usize, usize) {
    if f1.len() != f2.len() {
        panic!("Expected fingerprints to have same length.");
    }

    let update_counts = |(mut a, mut b, mut c), (&x, &y)| {
        match (x, y) {
            (1, 1) => c += 1,
            (1, _) => a += 1,
            (_, 1) => b += 1,
            _ => {}
        }
        (a, b, c)
    };

    f1.iter().zip(f2.iter()).fold((0, 0, 0), update_counts)
}

pub fn tanimoto(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    c as f64 / (a + b - c) as f64
}

pub fn euclidean(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    ((a + b - 2 * c) as f64).sqrt()
}

pub fn hamming(f1: &[u32], f2: &[u32]) -> usize {
    let (a, b, c) = abc(f1, f2);
    a + b - 2 * c
}

pub fn dice(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (2 * c) as f64 / (a + b) as f64
}

pub fn cosine(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    c as f64 / ((a * b) as f64).sqrt()
}

pub fn russell_rao(f1: &[u32], f2: &[u32]) -> f64 {
    let (_, _, c) = abc(f1, f2);
    c as f64 / f1.len() as f64
}

pub fn forbes(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (c * f1.len()) as f64 / (a * b) as f64
}

pub fn soergel(f1: &[u32], f2: &[u32]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (a + b - 2 * c) as f64 / (a + b - c) as f64
}
