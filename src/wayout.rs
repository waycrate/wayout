use smithay_client_toolkit::reexports::{
    client::{Display, GlobalManager},
    protocols::wlr::unstable::output_power_management::v1::client::{
        zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1, zwlr_output_power_v1,
        zwlr_output_power_v1::Mode,
    },
};

use std::{error::Error, process::exit};

mod clap;
mod output;

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = clap::set_flags().get_matches();
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);
    event_queue
        .dispatch(&mut (), |_, _, _| unreachable!())
        .unwrap();
    let valid_outputs = output::get_valid_outputs(display);

    let output_power_manager = globals.instantiate_exact::<ZwlrOutputPowerManagerV1>(1);
    if args.is_present("output") {
        let mut is_present = false;
        for device in valid_outputs.clone() {
            let (output_device, info) = device;
            if info.name == args.value_of("output").unwrap().trim() {
                is_present = true;
                if !args.is_present("state") {
                    println!("Error: No output state provided, use --state flag and set it to \"on\" or \"off\"");
                    exit(1);
                }
                let state = match args
                    .value_of("state")
                    .unwrap()
                    .trim()
                    .to_lowercase()
                    .as_str()
                {
                    "on" => Mode::On,
                    "off" => Mode::Off,
                    _ => {
                        println!("Error: Invalid state provided. Valid inputs: \"on\" \"off\"");
                        exit(1);
                    }
                };
                output_power_manager
                    .as_ref()
                    .unwrap()
                    .get_output_power(&output_device.clone())
                    .set_mode(state);
            }
        }
        if !is_present {
            println!(
                "\"{}\" is not a valid output.",
                args.value_of("output").unwrap().trim()
            );
            exit(1);
        }
    }

    for output in valid_outputs {
        if args.is_present("output") {
            break;
        }
        let (output, output_info) = output;
        let output_name = output_info.name;
        output_power_manager
            .as_ref()
            .unwrap()
            .get_output_power(&output)
            .quick_assign({
                let output_name = output_name.clone();
                move |_, event, _| match event {
                    zwlr_output_power_v1::Event::Mode { mode } => match mode {
                        Mode::On => {
                            println!("{} {:#?}", output_name, mode);
                        }
                        Mode::Off => {
                            println!("{} {:#?}", output_name, mode);
                        }
                        _ => unreachable!(),
                    },
                    zwlr_output_power_v1::Event::Failed {} => {
                        println!("Compositor returned Failed event.");
                        exit(1);
                    }
                    _ => unreachable!(),
                }
            });
    }
    event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;
    Ok(())
}
