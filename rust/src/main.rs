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


fn main() {
    println!("The faculty of {} is {}", 5, fac(5));
    println!("The result of the coin flip is {}", flip_coin());
    println!("The result of the weighted sampling is {}", choose_weighted());
}
