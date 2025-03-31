use std::env;

fn main() {
    let mut roll_request = rollerrs::Request {
        rolls: 1,
        sides: 20,
        with: rollerrs::With::None,
        bonus: 0,
    };

    let args: Vec<String> = env::args().collect();

    match rollerrs::parse_args(&mut roll_request, args[1].as_str()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {e}");
            return;
        },
    }

    let rolls = rollerrs::conduct_rolls(&roll_request);

    println!("{:?}", rolls);
}
