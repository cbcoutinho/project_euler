mod modulo;

fn main() {

    let end = 1000;

    let s: i32 = (1..end)
        .filter(|&x| modulo::is_multiple_3or5(&x))
        .sum();

    println!("Sum Multiples of 3 or 5 up to {} = {}", end, s);
}
