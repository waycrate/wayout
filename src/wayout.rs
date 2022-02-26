use smithay_client_toolkit::{
    output::OutputInfo,
    reexports::{
        client::{protocol::wl_output::WlOutput, Display, GlobalManager},
        protocols::wlr::unstable::output_power_management::v1::client::{
            zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1,
            zwlr_output_power_v1::Mode, zwlr_output_power_v1,
        },
    },
};

use std::{cell::RefCell, process::exit, rc::Rc};

mod flags;
mod output;

#[derive(Debug, Clone)]
pub struct HeadState {
    name: String,
    mode: Mode,
}

pub fn main() {
    let args = flags::set_flags().get_matches();
    let mut output_name:String = String::new();
    for flag in ["on", "off", "toggle"] {
        if args.is_present(flag) {
            output_name = args.value_of(flag).unwrap().trim().to_string();
        }
    }

    if args.is_present("on") {
        set_head_state(output_name.clone(), Mode::On);
        exit(1);
    }
    if args.is_present("off") {
        set_head_state(output_name.clone(), Mode::Off);
        exit(1);
    }
    if args.is_present("toggle") {
        let head_states = get_head_states();
        for head in head_states {
            if head.name == output_name.clone() {
                match head.mode {
                    Mode::On => {
                        set_head_state(output_name.clone(), Mode::Off);
                    },
                    Mode::Off => {
                        set_head_state(output_name.clone(), Mode::On);
                    },
                    _ => unreachable!()
                }
                exit(1);
            }
        }
    }

    // we exit from the previous flags on completion, if none of these flags are triggered then exit(1) isn't called so we can execute the print function now!
    for head in get_head_states(){
        println!("{} {:#?}", head.name, head.mode );
    }
}

pub fn set_head_state(output_name:String, mode: Mode){
    let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    let valid_outputs = output::get_valid_outputs(display);
    let output_power_manager = globals.instantiate_exact::<ZwlrOutputPowerManagerV1>(1);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    let output_choice = get_wloutput(output_name.to_string(), valid_outputs.clone());
    output_power_manager.as_ref().unwrap().get_output_power(&output_choice.clone()).set_mode(mode);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();
}
pub fn get_head_states() -> Vec<HeadState> {
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
                            head_states.borrow_mut().push(HeadState {
                                name: output_name.clone(),
                                mode: mode,
                            });
                        }
                        Mode::Off => {
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
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();
    let head_states = head_states.borrow_mut().to_vec();head_states
}

 fn get_wloutput(name: String, valid_outputs: Vec<(WlOutput, OutputInfo)>) -> WlOutput {
    for device in valid_outputs.clone() {
        let (output_device, info) = device;
        if info.name == name {
            return output_device;
        }
    }
    println!("Error: No output of name \"{}\" was found", name);
    exit(1);
}