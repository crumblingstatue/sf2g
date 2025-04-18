//! This is mainly an example to test linking solely against the SFML system module

use {
    sf2g::{
        SfError,
        system::{Clock, Time},
    },
    std::io::Write,
};

fn main() -> Result<(), SfError> {
    let clock = Clock::start()?;
    while clock.elapsed_time().as_seconds() < 5.0 {
        print!("Elapsed time: {}\r", clock.elapsed_time().as_seconds());
        let _ = std::io::stdout().flush();
        sf2g::system::sleep(Time::milliseconds(100));
    }
    Ok(())
}
