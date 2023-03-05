use futures::{stream::SelectAll, Stream, StreamExt};
pub use k8s_openapi::api::core::v1::{Pod, Service};
use kube::{
    api::{ListParams, LogParams},
    config::Kubeconfig,
    Api, Client, Config, ResourceExt,
};

#[cfg(not(feature = "mock"))]
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

#[cfg(not(feature = "mock"))]
pub fn list_ctx() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let ctxs = Kubeconfig::read()?
        .contexts
        .into_iter()
        .map(|x| x.name)
        .filter(|x| x.starts_with("betty-"))
        .collect();

    Ok(ctxs)
}

#[cfg(not(feature = "mock"))]
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

    let client = Client::try_from(config).map_err(|e| e.to_string())?;
    let pods: Api<Pod> = Api::namespaced(client, namespace);
    let mut app_pods = Vec::new();
    for p in pods
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

#[cfg(feature = "mock")]
pub async fn list_svc(namespace: &str) -> Result<Vec<Service>, String> {
    Ok(vec![
        generate_service("one"),
        generate_service("two"),
        generate_service("three"),
        generate_service("four"),
        generate_service("five"),
    ])
}

#[cfg(feature = "mock")]
fn generate_service(name: &str) -> Service {
    let mut service = Service::default();
    service.metadata.name = Some(name.to_string());

    service
}

#[cfg(feature = "mock")]
pub fn list_ctx() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok((0..10).map(|x| format!("ctx: {x}")).collect())
}

#[cfg(feature = "mock")]
pub async fn logger(
    ctx: &str,
    namespace: &str,
    app: Service,
) -> Result<impl Stream<Item = String>, String> {
    Ok(futures::stream::iter(
        (0..30).map(|x| format!("logging: {x}")),
    ))
}
