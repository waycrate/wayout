use clap::{arg, Command};

pub fn set_flags() -> Command<'static> {
    let app = Command::new("wayout")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Simple tool to set output mode of Heads for wlroots based compositors.")
        .arg(
            arg!(--off <OUTPUT>)
                .required(false)
                .takes_value(true)
                .conflicts_with("on")
                .conflicts_with("toggle")
                .help("Turn off the display."),
        )
        .arg(
            arg!(--on <OUTPUT>)
                .required(false)
                .takes_value(true)
                .conflicts_with("off")
                .conflicts_with("toggle")
                .help("Turn on the display."),
        )
        .arg(
            arg!(--toggle <OUTPUT>)
                .required(false)
                .takes_value(true)
                .conflicts_with("on")
                .conflicts_with("off")
                .help("Toggle the output state of the display."),
        );
    app
}
