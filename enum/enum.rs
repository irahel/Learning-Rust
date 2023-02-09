enum week
{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

fn is_weekday(day: week) -> bool
{
    match day
    {
        week::Saturday | week::Sunday => false,
        _ => true
    }
}
