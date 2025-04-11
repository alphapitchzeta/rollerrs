use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = match args.get(1) {
        Some(val) => val,
        None => {
            println!("Error: malformed argument");
            return;
        },
    };

    let roll_request = match rollerrs::parse_args(args.as_str()) {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {e}");
            return;
        },
    };

    let rolls = rollerrs::conduct_rolls(&roll_request);

    println!("{:?}", rolls);
}
