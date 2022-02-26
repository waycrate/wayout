use clap::{arg, Command};

pub fn set_flags() -> Command<'static> {
    let app = Command::new("wayout")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple tool to set output mode of Heads for wlroots based compositors.")
        .arg(
            arg!(-o --output <OUTPUT>)
                .required(false)
                .takes_value(true)
                .help("Choose a particular display to screenshot."),
        )
        .arg(
            arg!(-s --state <STATE>)
                .required(false)
                .takes_value(true)
                .help("Set output state to on or off."),
        )
        .arg(
            arg!(-t - -toggle)
                .required(false)
                .takes_value(false)
                .conflicts_with("state")
                .help("Toggle output state."),
        );
    app
}
