fn result() -> Result<String, u8>
{
    Ok(String::from("All good"))
    //Err(1)
}

fn main()
{
    /*
    let vet = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    vet[10]; index out of bounds: the len is 10 but the index is 10
    */

    match result()
    {
        Ok(s) => println!("Sucess: {}", s),
        Err(e) => println!("Error: {}", e)
    }

    panic!("This is a proposital panic");
}