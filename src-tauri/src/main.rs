// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use futures::StreamExt;
use kube::{config::Kubeconfig, ResourceExt};
use tauri::{
    async_runtime::{block_on, Mutex, TokioJoinHandle},
    Manager, State,
};

type KuberState = Arc<Mutex<KuberStateInner>>;

mod kuber;

struct KuberStateInner {
    ctxs: Vec<String>,
    svcs: Vec<kuber::Service>,
    selected_svc: Option<kuber::Service>,
    selected_ctx: Option<String>,
    join_handle: Option<TokioJoinHandle<()>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_contexts(state: State<'_, KuberState>) -> Result<Vec<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.ctxs.to_vec()
    };
    Ok(res)
}

#[tauri::command]
async fn set_ctx(state: State<'_, KuberState>, ctx: String) -> Result<(), ()> {
    let mut lock = state.lock().await;
    lock.selected_ctx = Some(ctx);

    Ok(())
}

#[tauri::command]
async fn get_ctx(state: State<'_, KuberState>) -> Result<Option<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.selected_ctx.clone()
    };

    Ok(res)
}

#[tauri::command]
async fn fetch_services(state: State<'_, KuberState>) -> Result<Vec<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.svcs.iter().map(|x| x.name_any()).collect()
    };
    Ok(res)
}

#[tauri::command]
async fn set_svc(state: State<'_, KuberState>, svc: String) -> Result<(), ()> {
    let mut lock = state.lock().await;
    let found = lock.svcs.iter().find(|x| x.name_any() == svc).cloned();
    lock.selected_svc = found;

    Ok(())
}

#[tauri::command]
async fn get_svc(state: State<'_, KuberState>) -> Result<Option<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.selected_svc.clone()
    };

    Ok(res.map(|x| x.name_any()))
}

#[tauri::command]
async fn log_messages(window: tauri::Window, state: State<'_, KuberState>) -> Result<(), String> {
    let res = {
        let lock = state.lock().await;
        (lock.selected_ctx.clone(), lock.selected_svc.clone())
    };

    if let (Some(ctx), Some(svc)) = res {
        let mut stream = kuber::logger(&ctx, "frontend", svc).await?;
        let win1 = window.clone();
        let join_handle = tokio::spawn(async move {
            while let Some(line) = stream.next().await {
                win1.emit("onMessage", line).unwrap();
            }
        });

        {
            let mut lock = state.lock().await;
            lock.join_handle = Some(join_handle)
        }
    }

    Ok(())
}

#[tauri::command]
async fn cancel_messages(state: State<'_, KuberState>) -> Result<(), String> {
    let mut lock = state.lock().await;
    if let Some(join_handle) = &lock.join_handle {
        join_handle.abort();
        lock.join_handle = None;
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let available_contexts: Vec<_> = Kubeconfig::read()?
                .contexts
                .into_iter()
                .map(|x| x.name)
                .filter(|x| x.starts_with("betty-"))
                .collect();

            let svcs = block_on(kuber::list_svc("frontend"))?;

            app.manage(Arc::new(Mutex::new(KuberStateInner {
                ctxs: available_contexts,
                svcs,
                selected_svc: None,
                selected_ctx: None,
                join_handle: None,
            })));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_contexts,
            set_ctx,
            get_ctx,
            log_messages,
            cancel_messages,
            fetch_services,
            set_svc,
            get_svc,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
