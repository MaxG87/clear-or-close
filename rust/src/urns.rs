use rand::distributions::*;
use rand::rngs::ThreadRng;

pub trait Urn {
    fn new(counts: &[u32]) -> Self;
    fn draw(&mut self, rng: &mut ThreadRng) -> usize;
}

pub struct ExactStdLibUrn {
    counts: Vec<u32>,
}

impl Urn for ExactStdLibUrn {
    fn new(counts: &[u32]) -> Self {
        let mut ret_self = Self {
            counts: Vec::with_capacity(counts.len()),
        };
        for elem in counts.iter() {
            ret_self.counts.push(*elem);
        }
        return ret_self;
    }

    fn draw(&mut self, mut rng: &mut ThreadRng) -> usize {
        let dist = WeightedIndex::new(self.counts.as_slice()).unwrap();
        let sampled_elem = dist.sample(&mut rng);
        self.counts[sampled_elem] -= 1;
        return sampled_elem;
    }
}
