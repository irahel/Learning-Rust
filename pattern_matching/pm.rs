fn main()
{
    for i in 1..=21
    {
        println!("{}: {}", i, match i
        {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            20 => "twenty",
            _ => "unknown"
        })
    }

    for i in 1..=20
    {
        println!("{}: {}", i, match i
        {
            1 | 2 => "low",
            3..=6 => "medium",
            7..=9 => "high",
            10 => "very high",
            _ if i % 2 == 0 => "even",
            _ => "odd"
        })
    }
}