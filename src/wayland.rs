use crate::output::{get_all_wl_outputs, OutputInfo};
use wayland_client::{
    delegate_noop,
    globals::{registry_queue_init, GlobalList, GlobalListContents},
    protocol::{
        wl_output::WlOutput,
        wl_registry::{self, WlRegistry},
    },
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols_wlr::output_power_management::v1::client::{
    zwlr_output_power_manager_v1::{self, ZwlrOutputPowerManagerV1},
    zwlr_output_power_v1::{self, Mode, ZwlrOutputPowerV1},
};

#[derive(Debug)]
pub struct WayoutConnection {
    pub wl_connection: Connection,
    pub wl_globals: GlobalList,
    pub wl_outputs: Vec<OutputInfo>,
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for WayoutConnection {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<WayoutConnection>,
    ) {
    }
}

impl WayoutConnection {
    pub fn init() -> Self {
        let connection = Connection::connect_to_env();
        if connection.is_err() {
            panic!(
                "Failed to connect to wayland socket: {}",
                connection.unwrap_err()
            );
        }
        let wl_connection = connection.unwrap();
        let (wl_globals, event_queue) =
            registry_queue_init::<WayoutConnection>(&wl_connection).unwrap();
        let queue_handle = event_queue.handle();

        let wayout_state = WayoutConnection {
            wl_connection,
            wl_globals,
            wl_outputs: vec![],
        };

        return wayout_state;
    }

    pub fn refresh_outputs(self: &mut Self) {
        self.wl_outputs = get_all_wl_outputs(&self.wl_connection);
    }

    pub fn get_wloutput(self: &mut Self, name: String) -> Option<WlOutput> {
        for output in self.wl_outputs.clone() {
            if output.name == name {
                return Some(output.wl_output);
            }
        }
        println!("Error: No output of name \"{}\" was found", name);
        None
    }

    pub fn set_output_state(self: &Self, output: WlOutput, mode: Mode) {
        let mut event_queue = self.wl_connection.new_event_queue::<PowerManagerState>();
        let qh = event_queue.handle();

        let zwlr_output_power_manager = match self
            .wl_globals
            .bind::<ZwlrOutputPowerManagerV1, _, _>(&qh, 1..=1, ())
        {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to bind to required wayaland global: {}", e);
            }
        };

        let mut state = PowerManagerState {};
        let _ = self.wl_connection.display().get_registry(&qh, ());
        event_queue.roundtrip(&mut state).unwrap();

        zwlr_output_power_manager
            .get_output_power(&output, &qh, ())
            .set_mode(mode);
        event_queue.roundtrip(&mut state).unwrap();
    }

    pub fn get_output_state(self: &Self) -> Vec<OutputState> {
        let mut event_queue = self.wl_connection.new_event_queue::<OutputState>();
        let qh = event_queue.handle();

        let zwlr_output_power_manager = match self
            .wl_globals
            .bind::<ZwlrOutputPowerManagerV1, _, _>(&qh, 1..=1, ())
        {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to bind to required wayland global: {}", e);
            }
        };
        let mut states: Vec<OutputState> = vec![];

        for output in self.wl_outputs.clone() {
            let mut state = OutputState {
                name: output.name,
                mode: Mode::Off,
            };
            zwlr_output_power_manager.get_output_power(&output.wl_output, &qh, ());

            states.push(state.clone());
            event_queue.blocking_dispatch(&mut state).unwrap();
        }

        //event_queue.roundtrip(&mut state).unwrap();
        return states;
    }
}

pub struct PowerManagerState;
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OutputState {
    pub name: String,
    pub mode: Mode,
}

impl Dispatch<ZwlrOutputPowerV1, ()> for OutputState {
    fn event(
        app_state: &mut Self,
        _: &ZwlrOutputPowerV1,
        event: zwlr_output_power_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<OutputState>,
    ) {
        app_state.mode = match event {
            zwlr_output_power_v1::Event::Mode { mode } => mode.into_result().unwrap(),
            zwlr_output_power_v1::Event::Failed { .. } => {
                println!("Failed to get output power mode!");
                Mode::Off
            }
            _ => {
                unreachable!()
            }
        };
    }
}
impl Dispatch<ZwlrOutputPowerManagerV1, ()> for OutputState {
    fn event(
        _: &mut Self,
        _: &ZwlrOutputPowerManagerV1,
        _: zwlr_output_power_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<OutputState>,
    ) {
    }
}

//delegate_noop!(OutputState: ignore ZwlrOutputPowerManagerV1);
delegate_noop!(PowerManagerState: ignore ZwlrOutputPowerManagerV1);
delegate_noop!(PowerManagerState: ignore ZwlrOutputPowerV1);
delegate_noop!(PowerManagerState: ignore WlRegistry);
