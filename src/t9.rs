use std::char;

pub fn next_character<S>(input: S) -> char
where
    S: Into<char>,
{
    let input: char = input.into();
    match input {
        // Two
        '2' => 'A',
        'A' => 'B',
        'B' => 'C',
        'C' => 'A',
        // Three
        '3' => 'D',
        'D' => 'E',
        'E' => 'F',
        'F' => 'D',
        // Four
        '4' => 'G',
        'G' => 'H',
        'H' => 'I',
        'I' => 'G',
        // Five
        '5' => 'J',
        'J' => 'K',
        'K' => 'L',
        'L' => 'J',
        // Six
        '6' => 'M',
        'M' => 'N',
        'N' => 'O',
        'O' => 'M',
        // Seven
        '7' => 'P',
        'P' => 'Q',
        'Q' => 'R',
        'R' => 'S',
        'S' => 'P',
        // Eight
        '8' => 'T',
        'T' => 'U',
        'U' => 'V',
        'V' => 'T',
        // Nine
        '9' => 'W',
        'W' => 'X',
        'X' => 'Y',
        'Y' => 'Z',
        'Z' => 'W',
        _ => 0 as char
    }
}

pub fn _next_letter<S>(input: S) -> char
where
    S: Into<String>,
{
    let input: String = input.into();
    next_character(input.chars().next().unwrap())
}
