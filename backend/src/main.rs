use crate::router::create_router;

mod health;
mod router;


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = create_router();

    Ok(router.into())
}

