// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data); // 传递引用而不是所有权

    let upper_case = string_uppercase(&data); // 传递引用而不是所有权

    println!("Last character: {}", last_char);
    println!("Uppercase string: {}", upper_case);
}

// 不获取所有权，返回最后一个字符
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 不获取所有权，将字符串转换为大写并返回
fn string_uppercase(data: &String) -> String {
    data.to_uppercase()
}
