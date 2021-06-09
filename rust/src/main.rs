use rand::distributions::*;
use rand::thread_rng;


/// Sample from Hypergeometric Distribution
///
/// This function takes a vector of counts and draws k elements from those
/// without replacement. It will return how often each element was drawn.
///
/// The returned vector will be hypergeometric distributed.
fn sample_hypergeometric(counts: Vec<u32>, k: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut ret_vec = vec![0; counts.len()];
    let mut counts_mut = counts;
    for _ in 0..k {
        let dist = WeightedIndex::new(&counts_mut).unwrap();
        let sampled_elem = dist.sample(&mut rng);
        counts_mut[sampled_elem] -= 1;
        ret_vec[sampled_elem] += 1;
    }
    return ret_vec;
}


fn main() {
    const N_ELLIGIBLE_VOTERS: u32 = 46_500_000;
    const TURNOUT: f64 = 0.7221;
    const N_VOTERS: u32 = (N_ELLIGIBLE_VOTERS as f64 * TURNOUT) as u32;

    println!(
        "The result of the hypergeometric sampling is {}",
        sample_hypergeometric(vec![N_ELLIGIBLE_VOTERS/2, N_ELLIGIBLE_VOTERS/2], N_VOTERS)[0]
    );
}
