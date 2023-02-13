fn multiplication_table(number:u8) {
    let mut count = 1;
    while count <= 10 {
        println!("{} x {} = {}", number, count, number * count);
        count += 1;
    }
}

fn multiplication_table_until(number:u8, max:u8) {
    let mut count = 1;
    loop {
        println!("{} x {} = {}", number, count, number * count);
        count += 1;
        if count > max {
            break;
        }
    }
}

fn main() {
    multiplication_table(5);
    multiplication_table_until(5, 50);
}