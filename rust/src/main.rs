use rand::distributions::*;
use rand::thread_rng;


/// Sample from Hypergeometric Distribution
///
/// This function takes a vector of counts and draws k elements from those
/// without replacement. It will return how often each element was drawn.
///
/// The returned vector will be hypergeometric distributed.
fn sample_hypergeometric(counts: Vec<u64>, k: u64) -> Vec<u64> {
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
    let n_elligible_voters = 46_500_000;
    let turnout: f64 = 0.7221;
    let n_voters: u32 = (n_elligible_voters as f64 * turnout).floor() as u32;

    println!(
        "The result of the hypergeometric sampling is {}",
        sample_hypergeometric(vec![n_elligible_voters/2, n_elligible_voters/2], n_voters as u64)[0]
    );
}
