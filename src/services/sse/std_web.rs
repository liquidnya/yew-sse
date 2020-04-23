use stdweb::Reference;
use stdweb::__js_raw_asm_bool;
use stdweb::js;
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::web::event::SocketErrorEvent;
use stdweb::web::event::SocketMessageEvent;
use stdweb::web::event::SocketOpenEvent;
use stdweb::web::EventTarget;
use stdweb_derive::ReferenceType;

pub use stdweb::web::event::IMessageEvent;
pub use stdweb::web::EventListenerHandle;
pub use stdweb::web::IEventTarget;

pub type MessageEvent = SocketMessageEvent;
pub type ErrorEvent = SocketErrorEvent;
pub type OpenEvent = SocketOpenEvent;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "EventSource")]
#[reference(subclass_of(EventTarget))]
pub struct EventSource(Reference);

impl IEventTarget for EventSource {}

impl EventSource {
    pub fn close(&self) {
        js! {
            @(no_return)
            @{self}.close();
        }
    }

    pub fn ready_state(&self) -> u16 {
        js!( return @{self}.readyState; ).try_into().unwrap()
    }
}

pub fn new_event_source(url: &str) -> Result<EventSource, &'static str> {
    let event_source = js! {
        try {
            return new EventSource(@{url});
        } catch(error) {
            return error;
        }
    };
    EventSource::try_from(js!( return @{event_source.as_ref()}; ))
        .map_err(|_| "couldn't aquire event source")
}

pub fn new_event_source_with_credentials(
    url: &str,
    credentials: bool,
) -> Result<EventSource, &'static str> {
    let event_source = js! {
        try {
            return new EventSource(@{url}, { withCredentials: @{credentials} });
        } catch(error) {
            return error;
        }
    };
    EventSource::try_from(js!( return @{event_source.as_ref()}; ))
        .map_err(|_| "couldn't aquire event source")
}

#[derive(Debug)]
pub struct ClosingEventListenerHandle(Option<EventListenerHandle>);

impl Drop for ClosingEventListenerHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.remove();
        }
    }
}

impl From<EventListenerHandle> for ClosingEventListenerHandle {
    fn from(handle: EventListenerHandle) -> Self {
        Self(Some(handle))
    }
}
