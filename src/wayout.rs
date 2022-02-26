use smithay_client_toolkit::{
    output::OutputInfo,
    reexports::{
        client::{protocol::wl_output::WlOutput, Display, GlobalManager},
        protocols::wlr::unstable::output_power_management::v1::client::{
            zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1, zwlr_output_power_v1,
            zwlr_output_power_v1::Mode,
        },
    },
};

use clap::ArgMatches;
use std::{cell::RefCell, error::Error, process::exit, rc::Rc};

mod flags;
mod output;

#[derive(Debug, Clone)]
pub struct HeadState {
    name: String,
    mode: Mode,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = flags::set_flags().get_matches();
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);
    event_queue
        .dispatch(&mut (), |_, _, _| unreachable!())
        .unwrap();
    let valid_outputs = output::get_valid_outputs(display);

    let output_power_manager = globals.instantiate_exact::<ZwlrOutputPowerManagerV1>(1);
    let head_states: Rc<RefCell<Vec<HeadState>>> = Rc::new(RefCell::new(Vec::new()));

    for output in valid_outputs.clone() {
        let (output, output_info) = output;
        let output_name = output_info.name;
        output_power_manager
            .as_ref()
            .unwrap()
            .get_output_power(&output)
            .quick_assign({
                let head_states = head_states.clone();
                let output_name = output_name.clone();
                move |_, event, _| match event {
                    zwlr_output_power_v1::Event::Mode { mode } => match mode {
                        Mode::On => {
                            println!("{} {:#?}", output_name, mode);
                            head_states.borrow_mut().push(HeadState {
                                name: output_name.clone(),
                                mode: mode,
                            });
                        }
                        Mode::Off => {
                            println!("{} {:#?}", output_name, mode);
                            head_states.borrow_mut().push(HeadState {
                                name: output_name.clone(),
                                mode: mode,
                            });
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

    if args.is_present("output") {
        if !args.is_present("state") && !args.is_present("toggle") {
            println!("Error: No output state provided, use --state flag and set it to \"on\" or \"off\". You can also use --toggle flag to toggle the current state.");
            exit(1);
        }
        let output_name = args.value_of("output").unwrap().trim();
        let output_choice =
            get_wloutput(output_name.to_string(), valid_outputs.clone(), args.clone()).clone();
        if args.is_present("state") {
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
                .get_output_power(&output_choice.clone())
                .set_mode(state);
        }
        if args.is_present("toggle") {
            for head in head_states.borrow_mut().to_vec() {
                if head.name == args.value_of("output").unwrap().trim() {
                    match head.mode {
                        Mode::On => {
                            output_power_manager
                                .as_ref()
                                .unwrap()
                                .get_output_power(&output_choice.clone())
                                .set_mode(Mode::Off);
                        }
                        Mode::Off => {
                            output_power_manager
                                .as_ref()
                                .unwrap()
                                .get_output_power(&output_choice.clone())
                                .set_mode(Mode::On);
                        }
                        _ => unreachable!(),
                    };
                    break;
                }
            }
        }
    }

    event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;
    Ok(())
}

pub fn get_wloutput(
    name: String,
    valid_outputs: Vec<(WlOutput, OutputInfo)>,
    args: ArgMatches,
) -> WlOutput {
    for device in valid_outputs.clone() {
        let (output_device, info) = device;
        if info.name == args.value_of("output").unwrap().trim() {
            return output_device;
        }
    }
    println!("Error: No output of name \"{}\" was found", name);
    exit(1);
}
