use rand::distributions::*;
use rand::thread_rng;


fn fac(n: u64) -> u64 {
    if n == 1 {
        1
    } else { n*fac(n-1) }

}


fn flip_coin() -> bool {
    let d = Bernoulli::new(0.5).unwrap();
    let v = d.sample(&mut thread_rng());
    return v
}

fn choose_weighted() -> char {
    let choices = ['a', 'b', 'c'];
    let weights = [1E10, 5E9, 5E9];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    choices[dist.sample(&mut rng)]
}


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
    println!("The faculty of {} is {}", 5, fac(5));
    println!("The result of the coin flip is {}", flip_coin());
    println!("The result of the weighted sampling is {}", choose_weighted());

    let n_elligible_voters = 46_500_000;
    let turnout: f64 = 0.7221;
    let n_voters: u32 = (n_elligible_voters as f64 * turnout).floor() as u32;

    println!(
        "The result of the hypergeometric sampling is {}",
        sample_hypergeometric(vec![n_elligible_voters/2, n_elligible_voters/2], n_voters as u64)[0]
    );
}
