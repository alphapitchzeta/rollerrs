use regex::Regex;

#[derive(Clone, Copy)]
pub enum With {
    Advantage,
    None,
    Disadvantage,
}

pub struct Request {
    pub rolls: i16,
    pub sides: i16,
    pub with: With,
    pub bonus: i16,
}

pub fn parse_args(r: &mut Request, args: &str) -> Result<(), String> {
    let main_regex =  match Regex::new(r"(?<rolls>\d{0,3})[d|D](?<sides>\d{1,3})(?<with>[a|d]?)") {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };

    let caps = match main_regex.captures(args) {
        Some(values) => values,
        None => return Err("could not parse args".to_string()),
    };

    let rolls: i16 = caps["rolls"].parse().unwrap_or(1);

    match rolls {
        1..=100 => (),
        _ => return Err("invalid roll value".to_string()),
    }

    let sides: i16 = match &caps["sides"].parse() {
        Ok(val) => match *val {
            1..=100 => *val,
            _ => return Err("invalid side value".to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };

    let with: With = match &caps["with"] {
        "d" | "D" => With::Disadvantage,
        "a" | "A" => With::Advantage,
        _ => With::None,
    };

    let bonus = 0;

    r.rolls = rolls;
    r.sides = sides;
    r.with = with;
    r.bonus = bonus;

    Ok(())
}

fn roll(sides: i16, w: With) -> i16 {
    let roll_a = rand::random_range(1..=sides);

    match w {
        With::None => roll_a,
        With::Advantage => {
            let roll_b: i16 = rand::random_range(1..=sides);
            roll_a.max(roll_b)
        },
        With::Disadvantage => {
            let roll_b: i16 = rand::random_range(1..=sides);
            roll_a.min(roll_b)
        },
    }
}

pub fn conduct_rolls(r: &Request) -> Vec<i16> {
    let mut rolls: Vec<i16> = Vec::new();

    for _ in 0..r.rolls {
        rolls.push(roll(r.sides, r.with) + r.bonus);
    }

    rolls
}
