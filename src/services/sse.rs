use super::Task;

use cfg_if::cfg_if;
use cfg_match::cfg_match;
use std::convert::TryInto;

cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
        use std_web::*;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
        use self::web_sys::*;
    }
}

impl Task for EventSourceTask {
    fn is_active(&self) -> bool {
        self.ready_state() != ReadyState::Closed
    }
}

impl EventSourceTask {
    pub fn ready_state(&self) -> ReadyState {
        let state = self.event_source.ready_state();
        match state {
            0 => ReadyState::Connecting,
            1 => ReadyState::Open,
            2 => ReadyState::Closed,
            _ => unreachable!("Unexpected value of EventSource::readyState: {}", state),
        }
    }
}

#[derive(Debug)]
pub struct EventSourceService {}

#[derive(PartialEq, Debug)]
pub enum ReadyState {
    Connecting,
    Open,
    Closed,
}

#[derive(Debug)]
#[must_use]
pub struct EventSourceTask {
    event_source: EventSource,
    handles: [EventListener; 3],
}

impl Drop for EventSourceTask {
    fn drop(&mut self) {
        self.event_source.close();
    }
}

#[derive(Debug)]
pub enum EventSourceUpdate {
    Error,
    Open,
}

impl EventSourceService {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn open<T>(
        &self,
        url: T,
        callback: yew::Callback<(String, String)>,
        updates: yew::Callback<EventSourceUpdate>,
    ) -> Result<EventSourceTask, &str>
    where
        T: TryInto<http::Uri>,
    {
        self.open_impl(
            new_event_source(
                &url.try_into()
                    .map_err(|_| "Could not parse url")?
                    .to_string(),
            )?,
            callback,
            updates,
        )
    }

    pub fn open_with_credentials<T>(
        &self,
        url: T,
        credentials: bool,
        callback: yew::Callback<(String, String)>,
        updates: yew::Callback<EventSourceUpdate>,
    ) -> Result<EventSourceTask, &str>
    where
        T: TryInto<http::Uri>,
    {
        self.open_impl(
            new_event_source_with_credentials(
                &url.try_into()
                    .map_err(|_| "Could not parse url")?
                    .to_string(),
                credentials,
            )?,
            callback,
            updates,
        )
    }

    fn open_impl(
        &self,
        event_source: EventSource,
        callback: yew::Callback<(String, String)>,
        updates: yew::Callback<EventSourceUpdate>,
    ) -> Result<EventSourceTask, &str> {
        let on_message = callback;
        let on_error = updates.clone();
        let on_open = updates;

        let message_handle = cfg_match! {
            feature = "std_web" => event_source.add_event_listener(move |event: MessageEvent| {
                    let text = event.data().into_text().expect("expect text data");
                    let id = event.last_event_id();
                    on_message.emit((id, text))
                }).into(),
            feature = "web_sys" => EventListener::new(&event_source, "message", move |event: &Event| {
                let event = event.dyn_ref::<MessageEvent>().unwrap();
                let text = event.data().as_string().expect("expect text data");
                let id = event.last_event_id();
                on_message.emit((id, text))
            }),
        };
        let error_handle = cfg_match! {
            feature = "std_web" => event_source.add_event_listener(move |_event: ErrorEvent| on_error.emit(EventSourceUpdate::Error)).into(),
            feature = "web_sys" => EventListener::new(&event_source, "error", move |_event: &Event| on_error.emit(EventSourceUpdate::Error)),
        };

        let open_handle = cfg_match! {
            feature = "std_web" => event_source.add_event_listener(move |_event: OpenEvent| on_open.emit(EventSourceUpdate::Open)).into(),
            feature = "web_sys" => EventListener::new(&event_source, "open", move |_event: &Event| on_open.emit(EventSourceUpdate::Open)),
        };
        Ok(EventSourceTask {
            event_source,
            handles: [message_handle, error_handle, open_handle],
        })
    }
}
