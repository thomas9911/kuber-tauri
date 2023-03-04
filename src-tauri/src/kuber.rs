use futures::{stream::SelectAll, Stream, StreamExt};
pub use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{
    api::{ListParams, LogParams},
    Api, Client, Config, ResourceExt,
};

pub async fn list_svc(namespace: &str) -> Result<Vec<Service>, String> {
    let client = Client::try_default().await.map_err(|e| e.to_string())?;
    let api: Api<Service> = Api::namespaced(client, namespace);

    let res = api
        .list(&ListParams::default())
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .collect();

    Ok(res)
}

pub async fn logger(
    ctx: &str,
    namespace: &str,
    app: Service,
) -> Result<impl Stream<Item = String>, String> {
    let config = Config::from_kubeconfig(&kube::config::KubeConfigOptions {
        context: Some(ctx.to_string()),
        ..Default::default()
    })
    .await
    .map_err(|e| e.to_string())?;

    // let config = Config::infer().await?;
    let client = Client::try_from(config).map_err(|e| e.to_string())?;

    // dbg!(&client.apiserver_version().await);

    let pods: Api<Pod> = Api::namespaced(client, namespace);
    let mut app_pods = Vec::new();
    for p in pods
        // .list(&old_select_way(app))
        // .await
        // .map_err(|e| e.to_string())?
        // .into_iter()
        // .chain(
        //     pods.list(&new_select_way(app))
        //         .await
        //         .map_err(|e| e.to_string())?,
        // )
        .list(&selector(&app))
        .await
        .map_err(|e| e.to_string())?
    {
        println!("found pod {:?}", p.name_any());
        app_pods.push(p.name_any());
    }

    let mut params = LogParams::default();
    params.pretty = true;
    params.follow = true;
    params.tail_lines = Some(50);

    let mut log_streams = SelectAll::new();

    for x in app_pods.iter() {
        log_streams.push(
            pods.log_stream(x, &params)
                .await
                .map_err(|e| e.to_string())?,
        )
    }

    Ok(log_streams.map(move |log| {
        let data = unify(
            log.map(|r| std::str::from_utf8(r.as_ref()).unwrap_or("").to_string())
                .map_err(|e| e.to_string()),
        );
        data
    }))
}

fn unify(res: Result<String, String>) -> String {
    match res {
        Ok(str) => str,
        Err(str) => str,
    }
}

// fn new_select_way(app: &str) -> ListParams {
//     ListParams::default().labels(&format!("app.kubernetes.io/name={app}"))
// }

// fn old_select_way(app: &str) -> ListParams {
//     ListParams::default().labels(&format!("app={app}"))
// }

fn selector(app: &Service) -> ListParams {
    if let Some(selector) = app.spec.as_ref().map(|x| x.selector.as_ref()).flatten() {
        let labels: Vec<String> = selector
            .iter()
            .map(|(key, val)| format!("{key}={val}"))
            .collect();
        ListParams::default().labels(&labels.join(","))
    } else {
        ListParams::default()
    }
}
