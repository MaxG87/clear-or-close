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
    accsum: Vec<u32>,
    sum: usize,
}

impl Urn for ExactStdLibUrn {
    fn new(counts: &[u32]) -> Self {
        let mut counts_copy = Vec::with_capacity(counts.len());
        for elem in counts {
            counts_copy.push(*elem);
        }
        Self { counts: counts_copy }
    }

    fn draw(&mut self, mut rng: &mut ThreadRng) -> usize {
        let dist = WeightedIndex::new(self.counts.as_slice()).unwrap();
        let sampled_elem = dist.sample(&mut rng);
        self.counts[sampled_elem] -= 1;
        sampled_elem
    }
}

impl Urn for ExactFastUrn {
    fn new(counts: &[u32]) -> Self {
        let mut accsum = Vec::with_capacity(counts.len());
        let mut sum = 0;
        for elem in counts {
            sum += elem;
            accsum.push(sum);
        }
        Self { accsum: accsum, sum: sum as usize }
    }

    fn draw(&mut self, mut rng: &mut ThreadRng) -> usize {
        let dist = Uniform::new_inclusive(0, self.sum);
        let point = dist.sample(&mut rng);
        let position = find_position(self.accsum.as_slice(), point);
        self.sum -= 1;
        for i in position..self.accsum.len() {
            self.accsum[i] -= 1;
        }
        position
    }
}

fn find_position(accsum: &[u32], point: usize) -> usize {
    for (i, elem) in accsum.iter().enumerate() {
        if *elem as usize >= point {
            return i;
        }
    }
    panic!("Sampling a valid position failed!")
}
