#[inline]
pub fn abc(f1: &[u8], f2: &[u8]) -> (usize, usize, usize) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 0..f1.len() {
        match (f1[i], f2[i]) {
            (1, 1) => c += 1,
            (1, _) => a += 1,
            (_, 1) => b += 1,
            _ => {}
        }
    }
    (a, b, c)
}

#[inline]
pub fn tanimoto(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    c as f64 / (a + b - c) as f64
}

#[inline]
pub fn euclidean(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    ((a + b - 2 * c) as f64).sqrt()
}

#[inline]
pub fn hamming(f1: &[u8], f2: &[u8]) -> usize {
    let (a, b, c) = abc(f1, f2);
    a + b - 2 * c
}

#[inline]
pub fn dice(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (2 * c) as f64 / (a + b) as f64
}

#[inline]
pub fn cosine(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    c as f64 / ((a * b) as f64).sqrt()
}

#[inline]
pub fn russell_rao(f1: &[u8], f2: &[u8]) -> f64 {
    let (_, _, c) = abc(f1, f2);
    c as f64 / f1.len() as f64
}

#[inline]
pub fn forbes(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (c * f1.len()) as f64 / (a * b) as f64
}

#[inline]
pub fn soergel(f1: &[u8], f2: &[u8]) -> f64 {
    let (a, b, c) = abc(f1, f2);
    (a + b - 2 * c) as f64 / (a + b - c) as f64
}
