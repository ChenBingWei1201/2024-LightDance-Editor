pub mod db;
pub mod global;
pub mod graphql;
pub mod server;
pub mod types;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Extension, Router,
};
use std::sync::Arc;

use std::fs;
use std::path::Path;

use crate::db::clients::{build_mysql_pool, build_redis_client};
use crate::global::APP_STATE;
use crate::graphql::schema::{build_schema, AppSchema};
use crate::graphql::subscriptor::ws::GraphQLSubscription;
use crate::server::ws::{ws_on_connect, ws_on_disconnect};
use crate::server::{extractors::Authentication, states::AppState};

async fn graphql(
    Authentication(context): Authentication,
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(context)).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/api/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv::dotenv().ok();

    let schema = build_schema().await;

    if let Err(write_error) = fs::write(Path::new("schema.graphql"), schema.sdl()) {
        println!("Error writing schema file: {}", write_error);
    }

    let mysql_pool = build_mysql_pool().await;
    let redis_client = build_redis_client().await;
    // Set database clients in global app state
    APP_STATE
        .set(Arc::new(AppState::new(mysql_pool, redis_client)))
        .unwrap();

    let app = Router::new()
        // Main routes for GraphQL
        .route("/api/graphql", get(graphiql).post(graphql))
        // Websocket route for GraphQL subscriptions with connection and disconnection callbacks
        .route(
            "/ws",
            get_service(GraphQLSubscription::new(ws_on_connect, ws_on_disconnect)),
        )
        // Pass schema as extension to routes
        .layer(Extension(schema));

    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "4000".to_string());

    println!("Server is ready!");
    println!("Listening on port {}", server_port);
    println!("GraphiQL: http://localhost:{}/api/graphql", server_port);

    axum::Server::bind(&format!("0.0.0.0:{}", server_port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
