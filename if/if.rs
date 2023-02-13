fn main() {
    let age:u8 = 16;
    let auth:bool = true;
    let legal_age = age >= 18 || (age >= 16 && auth);
    if legal_age {
        println!("You are old enough to party!");
    }
    else {
        println!("You are not old enough to party!");
    }

    println!("IF is an expression, not a statement and can be used to initialize variables");
    let condition = if legal_age { "of legal" } else { "not of legal" };
    println!("You are {} age", condition);
}