use shuttlings_cch24::router;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not migrate DB");

    let router = router().with_state(pool);

    Ok(router.into())
}
