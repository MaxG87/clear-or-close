use rand::distributions::*;
use rand::rngs::ThreadRng;

pub trait Urn {
    fn new(counts: &[u32]) -> Self;
    fn draw(&mut self, rng: &mut ThreadRng) -> usize;
}

pub struct ExactStdLibUrn {
    counts: Vec<u32>,
}

pub struct ExactFastUrn {
    counts: Vec<u32>,
    sum: usize,
}

impl Urn for ExactStdLibUrn {
    fn new(counts: &[u32]) -> Self {
        let mut ret_self = Self { counts: Vec::with_capacity(counts.len()) };
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

impl Urn for ExactFastUrn {
    fn new(counts: &[u32]) -> Self {
        let mut counts_copy = Vec::with_capacity(counts.len());
        let mut sum = 0;
        for elem in counts {
            sum += elem;
            counts_copy.push(*elem);
        }
        return Self { counts: counts_copy, sum: sum as usize };
    }

    fn draw(&mut self, mut rng: &mut ThreadRng) -> usize {
        let dist = Uniform::new(0, self.sum);
        let point = dist.sample(&mut rng);
        let position = 0usize;
        self.sum -= 1;
        self.counts[position] -= 1;
        return position;
    }
}
