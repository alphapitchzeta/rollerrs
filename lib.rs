pub enum With {
    Advantage,
    None,
    Disadvantage,
}

pub struct Request {
    pub rolls: i8,
    pub sides: i8,
    pub with: With,
    pub bonus: i8,
}

fn roll(sides: i8, w: &With) -> i8 {
    let roll_a = rand::random_range(1..=sides);

    match w {
        With::None => roll_a,
        With::Advantage => {
            let roll_b: i8 = rand::random_range(1..=sides);
            roll_a.max(roll_b)
        },
        With::Disadvantage => {
            let roll_b: i8 = rand::random_range(1..=sides);
            roll_a.min(roll_b)
        },
    }
}

pub fn conduct_rolls(r: &Request) -> Vec<i8> {
    let mut rolls: Vec<i8> = Vec::new();

    for _ in 0..r.rolls {
        rolls.push(roll(r.sides, &r.with) + r.bonus);
    }

    rolls
}