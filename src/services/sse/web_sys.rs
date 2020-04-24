pub use gloo_events::EventListener;
pub use wasm_bindgen::JsCast;
pub use web_sys::Event;
pub use web_sys::EventSource;
use web_sys::EventSourceInit;
pub use web_sys::MessageEvent;

pub fn new_event_source(url: &str) -> Result<EventSource, &'static str> {
    EventSource::new(url).map_err(|_| "couldn't aquire event source")
}

pub fn new_event_source_with_credentials(
    url: &str,
    credentials: bool,
) -> Result<EventSource, &'static str> {
    EventSource::new_with_event_source_init_dict(
        url,
        &EventSourceInit::new().with_credentials(credentials),
    )
    .map_err(|_| "couldn't aquire event source")
}
