pub fn fibonacci(i: u32) -> u32 {
    if i < 2 {
        1
    } else {
        fibonacci(i-2) + fibonacci(i-1)
    }
}
