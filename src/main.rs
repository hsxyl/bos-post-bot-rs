use std::{collections::HashMap, path::Path};

use anyhow::Ok;
use gql_client::Client;
use itertools::Itertools;
use near_units::{near::to_human, parse_near};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use user_action::UserAction;
use utils::*;
use workspaces::Account;
use workspaces::{result::ExecutionFinalResult, AccountId};

use crate::post::*;

pub mod post;
pub mod user_action;
pub mod utils;

#[derive(Serialize)]
struct Vars {
    last_timestamp: Option<String>,
    action_types: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub userActions: Vec<UserAction>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let endpoint = "https://api.thegraph.com/subgraphs/name/hsxyl/namesky-production";
    let query = r#"
    query GetUserActionsByUserId(
        $last_timestamp: BigInt = ""
        $action_types: [String]
      ) {
        userActions(
          first: 10
          orderBy: timestamp_plus_log_index
          orderDirection: desc
          where: { timestamp_gt: $last_timestamp, action_type_in: $action_types }
        ) {
          id
          user_id
          timestamp
          receipt_id

          contract_id
          token_id

          action_type

          create_listing_action {
            price
            listing_id
            listing {
              seller_id
              price
            }
          }
          # listing
          update_listing_action {
            old_price
            new_price
            listing {
              seller_id
              price
            }
          }
     
          buy_listing_action {
            buyer_id
            seller_id
            payout_balance
            payment_balance
          }
          # offer
          create_offering_action {
            offer_creator
            price
            offering_id
          }
          update_offering_action {
            offer_creator
            old_price
            new_price
            offering_id
          }
          accept_offering_action {
            buyer_id
            seller_id
            payout_balance # this is the amount the seller gets
            payment_balance # this is the amount of the offer
          }
      
          # nft
          nft_mint_action {
            token_id
            owner_id
          }
          nft_transfer_action {
            token_id
            old_owner_id
            new_owner_id
          }
          nft_burn_action {
            token_id
          }
        }
      }
    "#;

    let client = Client::new(endpoint);
    let vars = Vars {
        last_timestamp: Some(half_hour_ago_timestamp().to_string()),
        action_types: vec![
            "create_listing_action".to_string(),
            "update_listing_action".to_string(),
            "create_offering_action".to_string(),
            "update_offering_action".to_string(),
            "nft_mint_action".to_string(),
            // "buy_listing_action".to_string(),
            // "accept_offering_action".to_string(),
        ],
    };
    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap()
        .unwrap();

    dbg!(&data);

    let worker = workspaces::mainnet()
        .rpc_addr("https://1rpc.io/near")
        .await?;

    let account = Account::from_file(
        get_dir_path("bot.namesky.near"),
        &worker,
    )?;

    if data.userActions.len()>0 {
        let post_text =data.userActions
        .into_iter()
        .map(|user_action| build_post_text_by_user_action(user_action))
        .join("\n");
        send_post(&account, post_text).await.into_result()?;
    }
    
    Ok(())
}
