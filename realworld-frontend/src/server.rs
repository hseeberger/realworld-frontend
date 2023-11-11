use crate::app::App;
use anyhow::{Context, Result};
use axum::{
    body::{boxed, Body, BoxBody},
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
    Router, Server,
};
use leptos::{get_configuration, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{debug, error, info, warn};

pub async fn serve() -> Result<()> {
    let leptos_options = get_configuration(None)
        .await
        .context("get Leptos configuration")?
        .leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(leptos_options);

    info!(?addr, "running server");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("run server")
}

async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    _: Request<Body>,
) -> axum::response::Response {
    match get_static_file(&uri, &options.site_root).await {
        Ok(response) if response.status() == StatusCode::OK => response.into_response(),

        Ok(_) => {
            warn!(%uri, "cannot get file");
            StatusCode::NOT_FOUND.into_response()
        }

        Err(error) => {
            error!(?uri, error = format!("{error:#}"), "not found");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_static_file(uri: &Uri, site_root: &str) -> Result<Response<BoxBody>> {
    debug!(%uri, site_root, "getting static file");

    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .with_context(|| format!("build request for uri `{uri}`"))?;
    ServeDir::new(site_root)
        .oneshot(request)
        .await
        .with_context(|| format!("request to uri `{uri}`"))
        .map(|response| response.map(boxed))
}
