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
    zwlr_output_power_manager_v1::ZwlrOutputPowerManagerV1,
    zwlr_output_power_v1::{Mode, ZwlrOutputPowerV1},
};

#[derive(Debug)]
pub struct WayoutConnection {
    pub wl_connection: Connection,
    pub wl_globals: GlobalList,
    pub wl_outputs: Vec<OutputInfo>,
    pub zwlr_output_power_manager: Option<ZwlrOutputPowerManagerV1>,
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
            zwlr_output_power_manager: None,
        };

        let zxdg_output_power_manager =
            match wl_globals.bind::<ZwlrOutputPowerManagerV1, _, _>(&qh, 1..=1, ()) {
                Ok(x) => Some(x),
                Err(e) => {
                    print!("Failed to bind to required wayaland global: {}", e);
                    None
                }
            };

        let _ = wl_connection.display().get_registry(&queue_handle, ());

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

    pub fn set_head_state(self: &Self, output: WlOutput, mode: Mode) {
        let mut event_queue = self.wl_connection.new_event_queue::<PowerManagerState>();
        let qh = event_queue.handle();

        let zxdg_output_power_manager = match self
            .wl_globals
            .bind::<ZwlrOutputPowerManagerV1, _, _>(&qh, 1..=1, ())
        {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to bind to required wayaland global: {}", e);
                return;
            }
        };

        let mut state = PowerManagerState {};
        let _ = self.wl_connection.display().get_registry(&qh, ());
        event_queue.roundtrip(&mut state).unwrap();

        zxdg_output_power_manager
            .get_output_power(&output, &qh, ())
            .set_mode(mode);
        event_queue.roundtrip(&mut state).unwrap();
    }
}

pub struct PowerManagerState;
delegate_noop!(PowerManagerState: ignore ZwlrOutputPowerManagerV1);
delegate_noop!(PowerManagerState: ignore ZwlrOutputPowerV1);
delegate_noop!(PowerManagerState: ignore WlRegistry);
