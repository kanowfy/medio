use std::error::Error;

use wasmbus_rpc::{actor::prelude::*, minicbor::{decode, Encode, Decode}};
use wasmcloud_interface_sqldb::*;

use crate::newsletter::SubscribeRequest;

const TABLE_NAME: &str = "subscribers";

#[derive(Encode, Decode)]
struct SubscriberModel {
    // Only need email for now
    #[n(0)]
    email: String
}

pub(crate) async fn subscribe(ctx: &Context, req: &SubscribeRequest) -> Result<(), Box<dyn Error>> {
    let _resp = SqlDbSender::new().execute(
        ctx,
        &format!("INSERT INTO {} (email) VALUES ('{}')",
        TABLE_NAME, req.email.clone()).into()
    ).await?;

    Ok(())
}

pub(crate) async fn get_subscribers(ctx: &Context) -> Result<Vec<String>, Box<dyn Error>> {
    let resp = SqlDbSender::new().query(
        ctx,
        &format!("SELECT email FROM {}",
        TABLE_NAME).into()
    ).await?;

    if resp.num_rows == 0 {
        return Ok(vec![]);
    }

    let rows: Vec<SubscriberModel> = decode(&resp.rows)?;
    Ok(rows.into_iter().map(|s| s.email).collect::<Vec<String>>())
}