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
    let mut wayout_conn = WayoutConnection::init();
    wayout_conn.refresh_outputs();

    let output = wayout_conn.get_wloutput("WL-1".to_string()).unwrap();
    wayout_conn.set_output_state(output, Mode::On);

    for i in wayout_conn.wl_outputs.clone().into_iter() {
        println!("{}", i);
    }

    let v = wayout_conn.get_output_state();
    for head in v {
        println!("{} {:?}", head.name, head.mode);
    }
    return;

    let args = flags::parse_flags();
    let mut output_name = String::new();

    for arg in [&args.on, &args.off, &args.toggle] {
        if let Some(output) = arg {
            output_name = output.trim().to_string();
        }
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
    /* let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    let valid_outputs = output::get_all_outputs(display);
    let output_power_manager = globals.instantiate_exact::<ZwlrOutputPowerManagerV1>(1);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    let output_choice = output::get_wloutput(output_name, valid_outputs);
    output_power_manager
        .as_ref()
        .unwrap()
        .get_output_power(&output_choice)
        .set_mode(mode);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap(); */
}
pub fn get_head_states() -> Vec<HeadState> {
    /* let display = Display::connect_to_env().unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);
    event_queue
        .dispatch(&mut (), |_, _, _| unreachable!())
        .unwrap();
    let valid_outputs = output::get_all_outputs(display);
    let output_power_manager = globals.instantiate_exact::<ZwlrOutputPowerManagerV1>(1);
    let head_states: Rc<RefCell<Vec<HeadState>>> = Rc::new(RefCell::new(Vec::new()));

    for output in valid_outputs {
        let output_name = output.name;
        let output_ptr = &output.wl_output;
        output_power_manager
            .as_ref()
            .unwrap()
            .get_output_power(output_ptr)
            .quick_assign({
                let head_states = head_states.clone();
                let output_name = output_name.clone();
                move |_, event, _| match event {
                    zwlr_output_power_v1::Event::Mode { mode } => match mode {
                        Mode::On => {
                            head_states.borrow_mut().push(HeadState {
                                name: output_name.clone(),
                                mode,
                            });
                        }
                        Mode::Off => {
                            head_states.borrow_mut().push(HeadState {
                                name: output_name.clone(),
                                mode,
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
    let head_states = head_states.borrow_mut().to_vec();
    head_states */

    return vec![];
}
