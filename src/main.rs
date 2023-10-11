use std::env;
use std::{collections::HashMap, path::Path};

use anyhow::Ok;
use gql_client::Client;
use itertools::Itertools;
use near_units::{near::to_human, parse_near};
use near_workspaces::Account;
use near_workspaces::{result::ExecutionFinalResult, AccountId};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use user_action::UserAction;
use utils::*;

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
pub struct UserActionData {
    pub userActions: Vec<UserAction>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filepath = args[1].as_str();

    let nano_timestamp = if args.len() == 2 {
        read_u128(filepath)
    } else {
        args[2].parse()?
    };

    let data = query_user_action(nano_timestamp).await;

    dbg!(&data);

    let worker = near_workspaces::mainnet()
        .rpc_addr("https://1rpc.io/near")
        .await?;

    let account = Account::from_file(get_dir_path("bot.namesky.near"), &worker)?;

    if data.userActions.len() > 0 {
        let latest_timestamp = data
            .userActions
            .iter()
            .map(|user_action| u128::from_str_radix(user_action.timestamp.as_str(), 10).unwrap())
            .max();
        let post_text = data
            .userActions
            .into_iter()
            .map(|user_action| build_post_text_by_user_action(user_action))
            .join("\n");

        send_post(&account, post_text).await.into_result()?;
        if latest_timestamp.is_some() {
            write_u128(latest_timestamp.unwrap(), &filepath);
        }
    }

    Ok(())
}

pub async fn query_user_action(nano_timestamp: u128) -> UserActionData {
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
        last_timestamp: Some(nano_timestamp.to_string()),
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
        .query_with_vars::<UserActionData, Vars>(query, vars)
        .await
        .unwrap()
        .unwrap();
    data
}
