fn modulo(x: &i32, n: &i32) -> i32 {
    ((x % n) + n) % n
}

fn is_multiple_3(x: &i32) -> bool {
    modulo(x, &3) == 0
}

fn is_multiple_5(x: &i32) -> bool {
    modulo(x, &5) == 0
}

pub fn is_multiple_3or5(x: &i32) -> bool {
    is_multiple_3(x) || is_multiple_5(x)
}
