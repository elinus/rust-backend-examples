#![allow(unused)] // For early development
// region:  --- Modules

mod model;
mod web;
mod log;
mod error;
mod ctx;

pub use self::error::{Error, Result};

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_response_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use crate::web::routes_login::routes;
// endRegion:  --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    // let routes_rpc = rpc::routes(mm::clone())
    //     .route_layer(middleware::from_fn(mw_response_map));

    let routes_all = Router::new()
        .merge(routes_login::routes())
        // .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region:  --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> {:<12} - {addr}\n", "LISTENING");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        . unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endRegion:  --- Start Server

    Ok(())
}
