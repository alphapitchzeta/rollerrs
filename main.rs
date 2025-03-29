fn main() {
    let roll_request = rollerrs::Request {
        rolls: 1,
        sides: 20,
        with: rollerrs::With::None,
        bonus: 0,
    };

    let rolls = rollerrs::conduct_rolls(&roll_request);

    println!("{:?}", rolls);
}
