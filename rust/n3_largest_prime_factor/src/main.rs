// mod prime;
// use prime::{is_prime, is_prime_naive};
// use prime::is_prime_naive;

extern crate primal;

fn main() {

    // let n: u64 = 13195;
    let n: u64 = 600851475143;

    let vec = (1..n).filter(|&i| n % i == 0)
                    .filter(|&i| primal::is_prime(i))
                    .collect::<Vec<_>>();

    let vmax = (1..n).filter(|&i| n % i == 0)
                    .filter(|&i| primal::is_prime(i))
                    .max().unwrap();

    println!("vec: {:?}", vec);
    println!("vec max: {}", vmax);
}
