use wayland_protocols_wlr::output_power_management::v1::client::{
    //zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1, zwlr_output_power_v1,
    zwlr_output_power_v1::Mode,
};
mod flags;
mod output;
mod wayland;
use wayland::WayoutConnection;

#[derive(Debug, Clone)]
pub struct HeadState {
    name: String,
    mode: Mode,
}
pub fn main() {
    let args = flags::parse_flags();
    let mut output_name = String::new();

    for arg in [&args.on, &args.off, &args.toggle].into_iter().flatten() {
        output_name = arg.trim().to_string();
    }

    if args.on.is_some() {
        set_head_state(output_name, Mode::On);
        return;
    } else if args.off.is_some() {
        set_head_state(output_name, Mode::Off);
        return;
    } else if args.toggle.is_some() {
        let head_states = get_head_states();
        for head in head_states {
            if head.name == output_name.clone() {
                match head.mode {
                    Mode::On => {
                        set_head_state(output_name, Mode::Off);
                    }
                    Mode::Off => {
                        set_head_state(output_name, Mode::On);
                    }
                    _ => unreachable!(),
                }
                return;
            }
        }
    }

    for head in get_head_states() {
        println!("{} {:?}", head.name, head.mode);
    }
}

pub fn set_head_state(output_name: String, mode: Mode) {
    let mut wayout_conn = WayoutConnection::init();
    wayout_conn.refresh_outputs();

    if let Some(output) = wayout_conn.get_wloutput(output_name) {
        wayout_conn.set_output_state(output, mode);
    }
}

pub fn get_head_states() -> Vec<HeadState> {
    let mut wayout_conn = WayoutConnection::init();
    wayout_conn.refresh_outputs();

    let output_states = wayout_conn.get_output_states();

    output_states
        .into_iter()
        .map(|state| HeadState {
            name: state.name,
            mode: state.mode,
        })
        .collect()
}
