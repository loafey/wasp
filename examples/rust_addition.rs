pub extern "C" fn addition() -> i32 {
    let a = 5;
    let b = 11;
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn main() {}
