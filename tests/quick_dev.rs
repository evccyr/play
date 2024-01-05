#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=John").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    let login_req = hc.do_post(
        "/api/login",
        json!({
        "uname":"yash",
        "pwd":"passwd"
        }),
    );
    login_req.await?.print().await?;

    hc.do_get("/hello2/John2").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    let create_ticket_req = hc.do_post(
        "/api/tickets",
        json!({
        "title": "sample"
        }),
    );
    create_ticket_req.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    let delete_ticket_req = hc.do_delete("/api/tickets/2");
    delete_ticket_req.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
