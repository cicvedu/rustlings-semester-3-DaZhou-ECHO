// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


fn round_to_n(x: f64, n: usize) -> f64 {
    let scale = 10_f64.powi(n as i32);
    (x * scale).round() / scale
}

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    round_to_n(total / values.len() as f64, 3) // 四舍五入到 3 位小数
}


fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
// fn average(values: &[f64]) -> f64 {
//     let total = values.iter().sum::<f64>();
//     (total / values.len() as f64).round()
// }

// fn main() {
//     let values = [3.5, 0.3, 13.0, 11.7];
//     println!("{}", average(&values));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn returns_proper_type_and_value() {
//         assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.0);
//     }
// }
