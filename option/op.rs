use std::fs::File;
use std::io::Read;

fn read_file(path: String) -> Option<String>
{
    let mut file = match File::open(path)
    {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents)
    {
        Ok(_) => Some(contents),
        Err(_) => None,
    }
}

fn main()
{
    let content = read_file("test.txt".to_string());
    match &content
    {
        Some(content) => println!("{}", content),
        None => println!("Error reading file"),
    }
}