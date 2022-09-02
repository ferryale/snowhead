use snowhead::uci::Uci;

fn main() {
    match Uci::cmd_loop() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
