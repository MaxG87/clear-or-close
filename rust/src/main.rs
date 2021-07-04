use rand::rngs::ThreadRng;
use rand::thread_rng;

mod urns;

/// Sample from Hypergeometric Distribution
///
/// This function takes a vector of counts and draws k elements from those
/// without replacement. It will return how often each element was drawn.
///
/// The returned vector will be hypergeometric distributed.
fn sample_hypergeometric<GenericUrn: urns::Urn>(
    counts: &[u32],
    k: u32,
    mut rng: &mut ThreadRng,
) -> Vec<u32> {
    let mut ret_vec = vec![0; counts.len()];
    let mut urn = GenericUrn::new(counts);
    for _ in 0..k {
        let sampled_elem: usize = urn.draw(&mut rng);
        ret_vec[sampled_elem] += 1;
    }
    return ret_vec;
}

/// Simulate plenty of two-option-polls
///
/// Returns: number of polls with more extreme outcome than n_votes_majority_option.
fn monte_carlo_significance_test_for_binary_election<Urn: urns::Urn>(
    counts: &[u32],
    k: u32,
    nrounds: usize,
    n_votes_majority_option: u32,
) -> u32 {
    let mut is_extreme = 0;
    let mut rng = thread_rng();
    for _ in 0..nrounds {
        let result = sample_hypergeometric::<Urn>(&counts, k, &mut rng);
        let sampled_majority_votes: u32 = *result.iter().max().unwrap();
        if sampled_majority_votes >= n_votes_majority_option {
            is_extreme += 1;
        }
    }
    return is_extreme;
}

fn main() {
    const N_ROUNDS: usize = 90_000;
    // const N_ELLIGIBLE_VOTERS: u32 = 46_500_000;
    const N_ELLIGIBLE_VOTERS: u32 = 2_500;
    const TURNOUT: f64 = 0.7221;
    const SHARE_OF_LEAVE: f64 = 0.5189;
    const N_VOTERS: u32 = (N_ELLIGIBLE_VOTERS as f64 * TURNOUT) as u32;
    const N_LEAVE_VOTES: u32 = (SHARE_OF_LEAVE * (N_VOTERS as f64)) as u32;
    let counts: Vec<u32> = vec![N_ELLIGIBLE_VOTERS / 2; 2];

    println!("The simulations will use the following boundary conditions:");
    println!("N_ROUNDS: {}", N_ROUNDS);
    println!("N_ELLIGIBLE_VOTERS: {}", N_ELLIGIBLE_VOTERS);
    println!("N_LEAVE_VOTES: {}", N_LEAVE_VOTES);
    println!("N_VOTERS: {}", N_VOTERS);
    println!("SHARE_OF_LEAVE: {}", SHARE_OF_LEAVE);
    println!("TURNOUT: {}", TURNOUT);

    let n_extreme_stdlib = monte_carlo_significance_test_for_binary_election::<
        urns::ExactStdLibUrn,
    >(
        counts.as_slice(),
        N_VOTERS,
        N_ROUNDS,
        N_LEAVE_VOTES,
    );
    let n_extreme_fast = monte_carlo_significance_test_for_binary_election::<
        urns::ExactFastUrn,
    >(
        counts.as_slice(), N_VOTERS, N_ROUNDS, N_LEAVE_VOTES
    );
    let n_extreme = n_extreme_fast + n_extreme_stdlib - n_extreme_stdlib;
    let p = n_extreme as f64 / N_ROUNDS as f64;
    println!(
        "{} out of {} sampled polls had a more extreme result. (p = {}).",
        n_extreme, N_ROUNDS, p,
    );
}
