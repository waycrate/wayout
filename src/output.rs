use std::{cell::RefCell, process::exit, rc::Rc};
use wayland_client::{protocol::wl_output, protocol::wl_output::WlOutput, Display, GlobalManager};

#[derive(Debug, Clone)]
pub struct OutputInfo {
    pub wl_output: WlOutput,
    pub name: String,
}

pub fn get_all_outputs(display: Display) -> Vec<OutputInfo> {
    // Connecting to wayland environment.
    let mut event_queue = display.create_event_queue();
    let attached_display = (*display).clone().attach(event_queue.token());

    // Instantiating the global manager.
    let globals = GlobalManager::new(&attached_display);
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    let outputs: Rc<RefCell<Vec<OutputInfo>>> = Rc::new(RefCell::new(Vec::new()));

    // Fetch all outputs and it's name.
    globals
        .instantiate_exact::<WlOutput>(4)
        .expect("Failed to bind to wl_output global.")
        .quick_assign({
            let outputs = outputs.clone();
            move |output, event, _| {
                if let wl_output::Event::Name { name } = event {
                    outputs.borrow_mut().push(OutputInfo {
                        wl_output: output.detach(),
                        name,
                    });
                }
            }
        });
    event_queue.sync_roundtrip(&mut (), |_, _, _| {}).unwrap();

    if outputs.borrow().is_empty() {
        println!("Compositor did not advertise any wl_output devices!");
        exit(1);
    }
    outputs.take()
}

/// Get a wl_output object from the output name.
pub fn get_wloutput(name: String, outputs: Vec<OutputInfo>) -> WlOutput {
    for output in outputs {
        if output.name == name {
            return output.wl_output;
        }
    }
    println!("Error: No output of name \"{}\" was found", name);
    exit(1);
}
