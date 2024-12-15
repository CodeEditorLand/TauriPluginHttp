// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Access the HTTP client written in Rust.

pub use error::{Error, Result};
pub use reqwest;
use tauri::{
	Manager,
	Runtime,
	plugin::{Builder, TauriPlugin},
};

mod commands;
mod error;
mod scope;

pub(crate) struct Http {
	#[cfg(feature = "cookies")]
	cookies_jar:std::sync::Arc<reqwest::cookie::Jar>,
}

pub fn init<R:Runtime>() -> TauriPlugin<R> {
	Builder::<R>::new("http")
		.setup(|app, _| {
			let state = Http {
				#[cfg(feature = "cookies")]
				cookies_jar:std::sync::Arc::new(reqwest::cookie::Jar::default()),
			};

			app.manage(state);

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			commands::fetch,
			commands::fetch_cancel,
			commands::fetch_send,
			commands::fetch_read_body,
		])
		.build()
}
