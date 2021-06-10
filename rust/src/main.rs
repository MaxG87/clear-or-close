use rand::distributions::*;
use rand::thread_rng;


/// Sample from Hypergeometric Distribution
///
/// This function takes a vector of counts and draws k elements from those
/// without replacement. It will return how often each element was drawn.
///
/// The returned vector will be hypergeometric distributed.
fn sample_hypergeometric(counts: &[u32], k: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut ret_vec = vec![0; counts.len()];
    let mut counts_mut = counts.to_vec();
    for _ in 0..k {
        let dist = WeightedIndex::new(&counts_mut).unwrap();
        let sampled_elem = dist.sample(&mut rng);
        counts_mut[sampled_elem] -= 1;
        ret_vec[sampled_elem] += 1;
    }
    return ret_vec;
}


fn monte_carlo_significance_test_for_binary_election(
    counts: &[u32], k: u32, nrounds: usize, n_votes_majority_option: u32
) -> u32 {
    let mut is_extreme = 0;
    for _ in 0..nrounds {
        let result = sample_hypergeometric(&counts, k);
        let sampled_majority_votes: u32 = *result.iter().max().unwrap();
        if sampled_majority_votes >= n_votes_majority_option {
            is_extreme += 1;
        }
    }
    return is_extreme;
}

    println!(
        "The result of the hypergeometric sampling is {}",
        sample_hypergeometric(vec![N_ELLIGIBLE_VOTERS/2, N_ELLIGIBLE_VOTERS/2], N_VOTERS)[0]
    );
}
