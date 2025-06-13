use crate::measures::euclidean;
use ndarray::Array2;
use petal_clustering::{Fit, Optics};
use petal_neighbors::distance::Euclidean;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Bubble {
    objects: Vec<Vec<u8>>,
}

impl Bubble {
    pub fn n(&self) -> usize {
        self.objects.len()
    }

    pub fn d(&self) -> usize {
        self.objects[0].len()
    }

    pub fn rep(&self) -> Vec<f64> {
        self.objects
            .iter()
            .fold(vec![0; self.n()], |acc, x| {
                acc.into_iter().zip(x.iter()).map(|(a, b)| a + b).collect()
            })
            .iter()
            .map(|x| *x as f64 / self.n() as f64)
            .collect()
    }

    pub fn extent(&self) -> f64 {
        (self.objects.iter().fold(0.0, |acc, i| {
            acc + self.objects.iter().fold(0.0, |inner_acc, j| {
                inner_acc
                    + i.iter()
                        .zip(j.iter())
                        .map(|(a, b)| a - b)
                        .fold(0, |squaresum, x| squaresum + (x ^ 2)) as f64
            })
        }) / (self.n() * (self.n() - 1)) as f64)
            .sqrt()
    }

    pub fn nn_dist(&self, k: usize) -> f64 {
        (k as f64 / self.n() as f64).powf(1.0 / (self.d() as f64))
    }
}

pub fn compute_bubbles(x: Vec<Vec<u8>>, k: usize) -> Vec<Bubble> {
    let sample_objects: Vec<Vec<u8>> = x
        .choose_multiple(&mut rand::thread_rng(), k)
        .cloned()
        .collect();

    x.iter()
        .fold(vec![Bubble { objects: vec![] }; k], |mut acc, a| {
            acc[sample_objects
                .iter()
                .map(|x| euclidean(x, a))
                .enumerate()
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
                .unwrap()
                .0]
                .objects
                .push(a.clone());
            acc
        })
}

pub fn cluster(x: Vec<Vec<u8>>, k: usize, tolerance: f64, min_pts: usize) -> Vec<Vec<Vec<u8>>> {
    let bubbles: Vec<Bubble> = compute_bubbles(x.clone(), k);
    let rep = Array2::from_shape_vec(
        (bubbles.len(), x[0].len()),
        bubbles.iter().flat_map(|b| b.rep()).collect(),
    )
    .unwrap();

    let clustering = Optics::new(tolerance, min_pts, Euclidean::default()).fit(&rep, None);

    clustering
        .0
        .values()
        .map(|i| i.iter().map(|j| x[*j].clone()).collect())
        .collect()
}
