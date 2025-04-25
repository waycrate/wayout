use crate::output::{OutputInfo, get_all_wl_outputs};
use wayland_client::{
    Connection, Dispatch, QueueHandle, delegate_noop,
    globals::{GlobalList, GlobalListContents, registry_queue_init},
    protocol::{
        wl_output::WlOutput,
        wl_registry::{self, WlRegistry},
    },
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
        let (wl_globals, _) = registry_queue_init::<WayoutConnection>(&wl_connection).unwrap();
        let wl_outputs = get_all_wl_outputs(&wl_connection);

        WayoutConnection {
            wl_connection,
            wl_globals,
            wl_outputs,
        }
    }

    pub fn get_wloutput(&self, name: String) -> Option<WlOutput> {
        for output in self.wl_outputs.clone() {
            if output.name == name {
                return Some(output.wl_output);
            }
        }
        println!("Error: No output of name \"{}\" was found", name);
        None
    }

    /// Responsibility of calling function to roundtrip / dispatch
    fn toggle_head(&self, output_state: OutputState) {
        match output_state.mode {
            Mode::On => {
                output_state.zwlr_output_power_v1.set_mode(Mode::Off);
            }
            Mode::Off => {
                output_state.zwlr_output_power_v1.set_mode(Mode::On);
            }
            _ => unreachable!(),
        }
    }

    pub fn toggle_outputs(&self, output_states: Vec<OutputState>, output_name: String) {
        if output_name == "*" {
            for output_state in output_states {
                self.toggle_head(output_state);
            }
        } else {
            for output_state in output_states {
                if output_state.name == output_name.clone() {
                    self.toggle_head(output_state);
                    break;
                }
            }
        }

        let mut event_queue = self.wl_connection.new_event_queue::<PowerManagerState>();

        let mut state = PowerManagerState {};
        event_queue.roundtrip(&mut state).unwrap();
    }

    pub fn get_output_states(&self) -> Vec<OutputState> {
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
            let output_power_mode =
                zwlr_output_power_manager.get_output_power(&output.wl_output, &qh, ());

            let mut state = OutputState {
                name: output.name,
                zwlr_output_power_v1: output_power_mode,
                mode: Mode::Off,
            };

            event_queue.blocking_dispatch(&mut state).unwrap();
            states.push(state.clone());
        }

        states
    }

    pub fn set_output_state(&self, output_name: String, mode: Mode) {
        let output = self.get_wloutput(output_name);
        if output.is_none() {
            return;
        }

        let mut event_queue = self.wl_connection.new_event_queue::<PowerManagerState>();
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

        let mut state = PowerManagerState {};
        let _ = self.wl_connection.display().get_registry(&qh, ());
        event_queue.roundtrip(&mut state).unwrap();

        zwlr_output_power_manager
            .get_output_power(&output.unwrap(), &qh, ())
            .set_mode(mode);
        event_queue.roundtrip(&mut state).unwrap();
    }
}

pub struct PowerManagerState;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OutputState {
    pub name: String,
    pub mode: Mode,
    pub zwlr_output_power_v1: ZwlrOutputPowerV1,
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
