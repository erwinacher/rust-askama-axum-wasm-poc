use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use http::header::{CACHE_CONTROL, HeaderValue};
use tower_layer::Layer;
use askama::Template;


#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new("static").append_index_html_on_directories(false);

    // Route for "/"
    let app = Router::new()
        .route("/", get(index_handler))
        .nest_service(
            "/static",
            SetResponseHeaderLayer::if_not_present(
                CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=60"),
            )
            .layer(static_files),
        );

    let addr = "0.0.0.0:3008";
    println!("Axum server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
