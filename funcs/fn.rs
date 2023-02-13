fn sum(a:i32, b:i32) -> i32 {
    println!("a: {} + b: {} = {}", a, b, a+b);

    println!("The last expression is the return value, without ;");
    println!("The keyword return is just for early returns");

    a + b
}

fn main() {
    println!("By default functions dont return anything");
    println!("Sum = {}", sum(33, 36));
}