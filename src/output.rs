use std::fmt::Display;
use std::process::exit;
use wayland_client::Dispatch;
use wayland_client::{
    protocol::{
        wl_output::{self, WlOutput},
        wl_registry::{self, WlRegistry},
    },
    Connection, QueueHandle,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputInfo {
    pub wl_output: WlOutput,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct OutputCaptureState {
    pub outputs: Vec<OutputInfo>,
}

impl Display for OutputInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.description)
    }
}

impl Dispatch<WlRegistry, ()> for OutputCaptureState {
    fn event(
        state: &mut Self,
        wl_registry: &WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        /* > The name event is sent after binding the output object. This event
         * is only sent once per output object, and the name does not change
         * over the lifetime of the wl_output global. */

        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "wl_output" {
                if version >= 4 {
                    let output = wl_registry.bind::<wl_output::WlOutput, _, _>(name, 4, qh, ());
                    state.outputs.push(OutputInfo {
                        wl_output: output,
                        name: String::new(),
                        description: String::new(),
                    });
                }
            }
        }
    }
}

impl Dispatch<WlOutput, ()> for OutputCaptureState {
    fn event(
        state: &mut Self,
        wl_output: &WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let output: &mut OutputInfo =
            if let Some(output) = state.outputs.iter_mut().find(|x| x.wl_output == *wl_output) {
                output
            } else {
                // Ignore wl_output events from outputs we don't recognize
                return;
            };

        match event {
            wl_output::Event::Name { name } => {
                output.name = name;
            }
            wl_output::Event::Description { description } => {
                output.description = description;
            }
            _ => {}
        }
    }
}

pub fn get_all_wl_outputs(wl_connection: &Connection) -> Vec<OutputInfo> {
    let mut state = OutputCaptureState { outputs: vec![] };
    let mut event_queue = wl_connection.new_event_queue::<OutputCaptureState>();
    let qh = event_queue.handle();

    let _ = wl_connection.display().get_registry(&qh, ());

    event_queue.roundtrip(&mut state).unwrap(); // This one gets all wl_outputs
    event_queue.roundtrip(&mut state).unwrap(); // This one gets all the events of ALL the
                                                // wl_outputs

    if state.outputs.is_empty() {
        println!("Compositor didn't advertise any valid wl_outputs");
    }
    return state.outputs;
}

pub fn get_wloutput(name: String, outputs: Vec<OutputInfo>) -> WlOutput {
    for output in outputs {
        if output.name == name {
            return output.wl_output;
        }
    }
    println!("Error: No output of name \"{}\" was found", name);
    exit(1);
}
