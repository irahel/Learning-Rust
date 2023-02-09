fn main()
{
    println!("Exploring variables");

    let variable:i8 = 127;
    let variable_u:u8 = 255;
    let variable_not_t = 300;
    println!("variable = {}, size = {}", variable, std::mem::size_of_val(&variable));
    println!("variable_u = {}, size = {}", variable_u, std::mem::size_of_val(&variable_u));
    println!("variable_not_t = {}, size = {}", variable_not_t, std::mem::size_of_val(&variable_not_t));

    let variable_dec:f32 = 2.5;
    println!("variable_dec = {}, size = {}", variable_dec, std::mem::size_of_val(&variable_dec));

    let variable_bool:bool = true;
    println!("variable_bool = {}, size = {}", variable_bool, std::mem::size_of_val(&variable_bool));

    println!("By default variables are immutable!");

    let mut variable_bool_mut:bool = true;
    println!("variable_bool_mut before= {}, size = {}", variable_bool_mut, std::mem::size_of_val(&variable_bool_mut));
    variable_bool_mut = false;
    println!("variable_bool_mut after= {}, size = {}", variable_bool_mut, std::mem::size_of_val(&variable_bool_mut));

    let variable_char:char = 'c';
    println!("variable_char = {}, size = {}", variable_char, std::mem::size_of_val(&variable_char));
    let variable_char:char = 'b';
    println!("variable_char = {}, size = {}", variable_char, std::mem::size_of_val(&variable_char));

    println!("We can redeclare a variable");
}