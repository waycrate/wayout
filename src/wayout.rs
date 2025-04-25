use wayland_protocols_wlr::output_power_management::v1::client::zwlr_output_power_v1::Mode;
mod flags;
mod output;
mod wayland;
use wayland::WayoutConnection;

pub fn main() {
    let args = flags::parse_flags();
    let mut output_name = String::new();

    for arg in [&args.on, &args.off, &args.toggle].into_iter().flatten() {
        output_name = arg.trim().to_string();
    }

    let wayout_conn = WayoutConnection::init();

    if args.on.is_some() {
        wayout_conn.set_output_state(output_name, Mode::On);
        return;
    } else if args.off.is_some() {
        wayout_conn.set_output_state(output_name, Mode::Off);
        return;
    } else if args.toggle.is_some() {
        let output_states = wayout_conn.get_output_states();
        wayout_conn.toggle_outputs(output_states, output_name);
        return;
    }

    let outputs = wayout_conn.get_output_states();
    for output in outputs {
        println!("{} {:?}", output.name, output.mode);
    }
}
