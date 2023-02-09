fn steals(s:String)
{
    println!("I stole {}", s);
}

fn main()
{
    println!("Stack allocated string");
    let str_s = "Rahel";

    println!("Heap allocated string");
    let str_h = String::from("Rahel");

    println!("{}, {}", str_s, str_h);
    println!("Stealing the heap allocated string");
    steals(str_h);

    // println!("{}", str_h); // Error: value borrowed here after move

    println!("Values have only one owner");
    println!("Copy values on the heap is not cheap");
}