fn fac(n: u64) -> u64 {
    if n == 1 {
        1
    } else { n*fac(n-1) }

}
fn main() {
    println!("The faculty of {} is {}", 5, fac(5));
}
