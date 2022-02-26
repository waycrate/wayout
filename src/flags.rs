use clap::{arg, Command};

pub fn set_flags() -> Command<'static> {
    let app = Command::new("wayout")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple tool to set output mode of Heads for wlroots based compositors.")
        .arg(
            arg!(--on <OUTPUT>)
                .required(false)
                .takes_value(true)
                .help("Set output state to on."),
        )
        .arg(
            arg!(--off <OUTPUT>)
                .required(false)
                .takes_value(true)
                .help("Set output state to off."),
        )
        .arg(
            arg!(--toggle <OUTPUT>)
                .required(false)
                .takes_value(true)
                .conflicts_with("state")
                .help("Toggle output state."),
        );
    app
}
