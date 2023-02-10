fn one_or_none() -> Option<i32>
{
    Some(1)
}

fn main()
{
    let content = one_or_none();

    if let Some(value) = content
    {
        println!("{:?}", content);
    }

    //While let is a loop, if let is a conditional
}