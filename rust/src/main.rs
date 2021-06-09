use rand::distributions::*;


fn fac(n: u64) -> u64 {
    if n == 1 {
        1
    } else { n*fac(n-1) }

}


fn flip_coin() -> bool {
    let d = Bernoulli::new(0.5).unwrap();
    let v = d.sample(&mut rand::thread_rng());
    return v
}


fn main() {
    println!("The faculty of {} is {}", 5, fac(5));
    println!("The result of the coin flip is {}", flip_coin());
}
