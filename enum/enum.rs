enum Week
{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

enum Color
{
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    CYMK{cyan:u8, magenta:u8, yellow:u8, black:u8}
}

fn print_color()
{
    let c = Color::RGB(0, 0, 0);
    println!("Color: {}", match c
    {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
        Color::RGB(0, 0, 0) => "Black",
        Color::RGB(_, _, _) => "rgb",
        Color::CYMK{cyan:_, magenta:_, yellow:_, black:_} => "cmyk"
    })

}

fn is_weekday(day: Week) -> bool
{
    match day
    {
        Week::Saturday | Week::Sunday => false,
        _ => true
    }
}

fn main()
{

}