fn foo()
{
    let example:i8 = 123;
    {
        let example:i8 = -123;
        println!("foo: {} in", example);
    }
    println!("foo: {} out", example);
}

fn bar()
{
    let example:u8 = 111;
    println!("bar: {}", example);
}

fn main() 
{
    foo();
    bar();
}