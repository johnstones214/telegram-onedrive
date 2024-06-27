/*
:project: telegram-onedrive
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

mod client;
mod env;
mod error;
mod handlers;
mod listener;
mod macros;
mod state;
mod trace;

use handlers::{help, start};
use listener::{EventType, Listener};
use trace::trace_registor;

#[tokio::main]
async fn main() {
    let _worker_guard = trace_registor();

    Listener::new()
        .await
        .on(EventType::command(start::PATTERN), start::handler)
        .on(EventType::command(help::PATTERN), help::handler)
        .run()
        .await;
}
