// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// question

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone()); //使用clone()保留原始data，和dataframe一样

    string_uppercase(data.clone());
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap() //没分号，没有所有权=>return
}

// Should take ownership
fn string_uppercase(mut data: String) {

    data = data.to_uppercase();
    println!("{}", data); //有分号，拥有所有权。

}
