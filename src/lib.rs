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

    let bonus_regex = match Regex::new(r"[d|D]\d{1,3}[a|d]?(?<sign>[+|-])(?<bonus>\d{1,3})") {
        Ok(val) => val,
        Err(e) => return Err(e.to_string()),
    };

    let main_caps = match main_regex.captures(args) {
        Some(values) => values,
        None => return Err("could not parse args".to_string()),
    };

    let rolls: i16 = main_caps["rolls"].parse().unwrap_or(1);

    match rolls {
        1..=100 => (),
        _ => return Err("invalid roll value".to_string()),
    }

    let sides: i16 = match &main_caps["sides"].parse() {
        Ok(val) => match *val {
            2..=100 => *val,
            _ => return Err("invalid side value".to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };

    let with: With = match &main_caps["with"] {
        "d" | "D" => With::Disadvantage,
        "a" | "A" => With::Advantage,
        _ => With::None,
    };

    let mut bonus = 0;

    if bonus_regex.is_match(args) {
        let bonus_caps = match bonus_regex.captures(args) {
            Some(values) => values,
            None => return Err("could not parse args".to_string()),
        };

        let sign = &bonus_caps["sign"];

        bonus = bonus_caps["bonus"].parse().unwrap_or_default();

        match bonus {
            1..=100 => (),
            _ => return Err("invalid bonus value".to_string()),
        }

        if sign == "-" {
            bonus *= -1;
        }
    }


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
