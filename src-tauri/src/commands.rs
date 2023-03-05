use futures::StreamExt;
use kube::ResourceExt;
use tauri::State;

use crate::kuber;
use crate::KuberState;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn fetch_contexts(state: State<'_, KuberState>) -> Result<Vec<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.ctxs.to_vec()
    };
    Ok(res)
}

#[tauri::command]
pub async fn set_ctx(state: State<'_, KuberState>, ctx: String) -> Result<(), ()> {
    let mut lock = state.lock().await;
    lock.selected_ctx = Some(ctx);

    Ok(())
}

#[tauri::command]
pub async fn get_ctx(state: State<'_, KuberState>) -> Result<Option<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.selected_ctx.clone()
    };

    Ok(res)
}

#[tauri::command]
pub async fn fetch_services(state: State<'_, KuberState>) -> Result<Vec<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.svcs.iter().map(|x| x.name_any()).collect()
    };
    Ok(res)
}

#[tauri::command]
pub async fn set_svc(state: State<'_, KuberState>, svc: String) -> Result<(), ()> {
    let mut lock = state.lock().await;
    let found = lock.svcs.iter().find(|x| x.name_any() == svc).cloned();
    lock.selected_svc = found;

    Ok(())
}

#[tauri::command]
pub async fn get_svc(state: State<'_, KuberState>) -> Result<Option<String>, ()> {
    let res = {
        let lock = state.lock().await;
        lock.selected_svc.clone()
    };

    Ok(res.map(|x| x.name_any()))
}

#[tauri::command]
pub async fn log_messages(
    window: tauri::Window,
    state: State<'_, KuberState>,
) -> Result<(), String> {
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
pub async fn cancel_messages(state: State<'_, KuberState>) -> Result<(), String> {
    let mut lock = state.lock().await;
    if let Some(join_handle) = &lock.join_handle {
        join_handle.abort();
        lock.join_handle = None;
    }

    Ok(())
}
