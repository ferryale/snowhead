use self::command::{GoOptions, UciCommand};
use self::option::UciOptions;
use crate::position::Position;
use crate::search::thread::SearchThread;
use crate::timeman::TimeManager;
use std::io;
use std::thread;
use std::time::SystemTime;

pub mod command;
pub mod option;

pub struct Uci;

/* Uci implementation */
impl Uci {
    // Handles go command to start search based on go_options and uci_options
    fn go(mut pos: Position, go_options: GoOptions, uci_options: UciOptions) {
        thread::spawn(move || {
            let time_manager = TimeManager::new(
                SystemTime::now(),
                &go_options,
                &uci_options,
                pos.side_to_move(),
            );
            let mut thread = SearchThread::new(go_options, time_manager);
            thread.search(&mut pos);
        });
    }

    // Cmd line uci loop
    pub fn cmd_loop() -> io::Result<()> {
        let mut uci_options = UciOptions::new();
        let mut position = Position::new().uci_options(&uci_options).build();

        loop {
            let uci_command = UciCommand::parse(&mut uci_options)?;

            match uci_command {
                UciCommand::Uci => {
                    println!("id name Snowhead v0.2.0");
                    println!("uciok");
                }
                UciCommand::Debug => {}
                UciCommand::Display => println!("{}", position.board()),
                UciCommand::IsReady => println!("readyok"),
                UciCommand::SetOption(uci_options) => println!("{:?}", uci_options),
                UciCommand::UciNewGame => {}
                UciCommand::Position(pos) => {
                    position = pos;
                }
                UciCommand::Go(go_options) => Self::go(position.clone(), go_options, uci_options),
                UciCommand::Stop => break,
                UciCommand::Ponderhit => {}
                UciCommand::Textel(textel_batch) => {
                    println!("{}", textel_batch.evaluate(&uci_options))
                }
                UciCommand::Quit => break,
                UciCommand::Invalid(invalid_cmd) => println!("Invalid command '{}'", invalid_cmd),
            }; // match
        } // loop

        Ok(())
    }
}
