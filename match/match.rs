fn main()
{
    let lang = "Rust";
    let purpose = match lang
    {
        "Rust" => "Systems Programming",
        "Python" => "Data Science",
        "Go" => "Cloud",
        "C#" => "Web",
        _ => "Unknown"
    };
    println!("{} is used for {}", lang, purpose);
}