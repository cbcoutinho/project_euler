mod fibonacci;
use fibonacci::fibonacci;

fn main() {

    let fib: u32 = (0..).map(|e| fibonacci(e))
                    .filter(|x| x % 2 == 0)
                    .take_while(|&x| x < 4_000_000)
                    .sum();

    println!("fib : {:?}", fib);

}
