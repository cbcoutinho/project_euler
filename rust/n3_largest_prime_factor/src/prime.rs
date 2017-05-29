// use std::iter::repeat;
//
// fn aks_coefficients(k: usize) -> Vec<i64> {
//     let mut coefficients = repeat(0i64).take(k + 1).collect::<Vec<_>>();
//     coefficients[0] = 1;
//     for i in 1..(k + 1) {
//         coefficients[i] = -(1..i).fold(coefficients[0], |prev, j|{
//             let old = coefficients[j];
//             coefficients[j] = old - prev;
//             old
//         });
//     }
//     coefficients
// }
//
// pub fn is_prime(p: usize) -> bool {
//     if p < 2 {
//         false
//     } else {
//         let c = aks_coefficients(p);
//         (1 .. (c.len() - 1) / 2 + 1).all(|i| (c[i] % (p as i64)) == 0)
//     }
// }

pub fn is_prime_naive(numb: usize) -> bool {
    let mut i: usize = 3;
    while i < numb {
        if numb % &i == 0 {
            return false
        }
        i = i + 2;
    }
    return true;
}
