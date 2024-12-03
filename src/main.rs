use shuttlings_cch24::router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router();

    Ok(router.into())
}
