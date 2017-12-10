mod digits;
use digits::Digits;

fn is_palindrome(n: usize) -> bool {
    let digits: Vec<usize> = Digits::new(n).collect();

    println!("{:?}", digits);
    println!("len(digits) = {}", digits.len());
    println!("len(digits)/2 = {}", digits.len()/2);

    let mut exit = true;

    for i in 0..digits.len()/2 {
        let dig_i = digits[i];
        let dig_ni = digits[digits.len()-i-1];

        println!("digits({}) = {}", i, &dig_i);
        println!("digits({}) = {}", digits.len()-i-1, &dig_ni);

        if &dig_ni != &dig_i { exit = false };
    }

    return exit
}

fn main() {

    println!("is_palindrome({}): {}", 421, is_palindrome(421));

    let v = (1..).take_while(|&x| x < 100)
                .filter(|&x| is_palindrome(x))
                .collect::<Vec<usize>>();

    println!("v: {:?}", v);

}
