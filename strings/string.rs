fn steals(s:String)
{
    println!("I stole {}", s);
}

fn steals_ref(s:&String)
{
    println!("References are immutable by default");
    println!("I not stole {}", s);
}

fn steals_ref_and_change(s:&mut String)
{
    println!("But we can use functions if mut is specified");
    s.push_str(" is my name");
    println!("I not stole {}", s);
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

    println!("An alternative is to use references");
    let str_h = String::from("Rahel");
    steals_ref(&str_h);

    println!("Now we can use the string again");
    println!("{}", str_h);

    let mut str_h = String::from("Rahel");
    steals_ref_and_change(&mut str_h);
    println!("{}", str_h);
}