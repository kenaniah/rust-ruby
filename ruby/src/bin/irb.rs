use std::io::{self, Write};
use rustyline::Editor;
use rustyline::error::ReadlineError;

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
/// Run a REPL interpreter
fn main() -> Result<(), Error> {

    let mut output = io::stdout();
    let mut error = io::stderr();
    let history_file = format!("{}/.irb_history", std::env::var("HOME").unwrap());

    // Readline
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(&history_file);

    // REPL
    loop {
        let readline = rl.readline("rust-irb(main):001:0> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                writeln!(output, "Line: {}", line).map_err(Error::Io)?;
            },
            Err(ReadlineError::Interrupted) => {
                writeln!(output, "Ctrl-C").map_err(Error::Io)?;
                break;
            },
            Err(ReadlineError::Eof) => {
                writeln!(output, "Ctrl-D").map_err(Error::Io)?;
            },
            Err(err) => {
                writeln!(error, "Error: {:?}", err).map_err(Error::Io)?;
            }
        }
    }

    // Attempt to save the history file
    let _ = rl.save_history(&history_file);

    // Exit
    Ok(())

}

#[derive(Debug)]
pub enum Error {
    ///IO error when writing to output or error streams
    Io(io::Error)
}
