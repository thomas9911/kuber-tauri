// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri::{
    async_runtime::{block_on, Mutex, TokioJoinHandle},
    Manager,
};

type KuberState = Arc<Mutex<KuberStateInner>>;

mod commands;
mod kuber;

pub struct KuberStateInner {
    ctxs: Vec<String>,
    svcs: Vec<kuber::Service>,
    selected_svc: Option<kuber::Service>,
    selected_ctx: Option<String>,
    join_handle: Option<TokioJoinHandle<()>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let ctxs = kuber::list_ctx()?;
            let svcs = block_on(kuber::list_svc("frontend"))?;

            app.manage(Arc::new(Mutex::new(KuberStateInner {
                ctxs,
                svcs,
                selected_svc: None,
                selected_ctx: None,
                join_handle: None,
            })));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fetch_contexts,
            commands::set_ctx,
            commands::get_ctx,
            commands::log_messages,
            commands::cancel_messages,
            commands::fetch_services,
            commands::set_svc,
            commands::get_svc,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
