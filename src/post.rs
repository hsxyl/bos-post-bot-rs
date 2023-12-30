use crate::*;
use anyhow::{anyhow, Ok};
use near_crypto::InMemorySigner;
use near_gas::NearGas;
use near_jsonrpc_client::methods;
use near_primitives::{
    transaction::{Action, FunctionCallAction, Transaction},
    types::{AccountId, BlockReference},
};

const NEAR_SOCIAL_CONTRACT_ID: &str = "social.near";
const NAMESKY_DOMAIN: &str = "namesky.app";

pub fn get_nft_url(token_id: &str) -> String {
    format!("https://{}/{}", NAMESKY_DOMAIN, token_id).to_string()
}

/*
{
  "data": {
    "autoctopus.near": {
      "post": {
        "main": "{\"type\":\"md\",\"text\":\"test\"}"
      },
      "index": {
        "post": "{\"key\":\"main\",\"value\":{\"type\":\"md\"}}"
      }
    }
  }
}
*/

pub fn render_post_json(account_id: AccountId, post_text: String) -> Value {
    json!({
        "data": json!({
            account_id: json!({
                "post": json!({
                    "main": json!({"type": "md", "text": post_text}).to_string(),
                }),
                "index": json!({
                    "post": json!({
                        "key": "main",
                        "value": json!({"type": "md"})
                    }).to_string()
                })
            })
        }),
    })
}

pub fn build_post_text_by_user_action(user_action: UserAction) -> String {
    match user_action.action_type {
        user_action::ActionType::create_listing_action => {
            format!(
                "{} is selling for {}. [Buy it now]({})!",
                user_action.token_id,
                to_human(user_action.create_listing_action.unwrap().price),
                get_nft_url(user_action.token_id.as_str())
            )
        }
        user_action::ActionType::update_listing_action => {
            format!(
                "{} is selling for {}. [Buy it now]({})!",
                user_action.token_id,
                to_human(user_action.update_listing_action.unwrap().new_price),
                get_nft_url(user_action.token_id.as_str())
            )
        }
        user_action::ActionType::create_offering_action => {
            format!(
                "{} received an offer for {}. [Check it now]({})!",
                user_action.token_id,
                to_human(user_action.create_offering_action.unwrap().price),
                get_nft_url(user_action.token_id.as_str())
            )
        }
        user_action::ActionType::update_offering_action => {
            format!(
                "{} received an offer for {}. [Check it now]({})!",
                user_action.token_id,
                to_human(user_action.update_offering_action.unwrap().new_price),
                get_nft_url(user_action.token_id.as_str())
            )
        }
        user_action::ActionType::buy_listing_action => {
            format!(
                "🎉🎉🎉Congratulations on [{}]({}) sold at {} !",
                user_action.token_id,
                get_nft_url(user_action.token_id.as_str()),
                to_human(user_action.buy_listing_action.unwrap().payment_balance)
            )
        }
        user_action::ActionType::accept_offering_action => {
            format!(
                "🎉🎉🎉Congratulations on [{}]({}) sold at {} !",
                user_action.token_id,
                get_nft_url(user_action.token_id.as_str()),
                convert_near_amount_to_human_readable(
                    user_action.accept_offering_action.unwrap().payment_balance
                )
            )
        }
        user_action::ActionType::nft_mint_action => {
            format!(
                "{} just mint into NFT. [Check it now]({})!",
                user_action.token_id,
                get_nft_url(user_action.token_id.as_str())
            )
        }
    }
    .to_string()
}

// pub async fn send_post(signer: &Account, post_text: String) -> ExecutionFinalResult {
//     let contract_id = NEAR_SOCIAL_CONTRACT_ID.parse().unwrap();
//     signer
//         .call(&contract_id, "set")
//         .args_json(render_post_json(signer.id().clone(), post_text))
//         .gas(NearGas::from_tgas(50))
//         .transact()
//         .await
//         .unwrap()
// }

pub async fn send_post(
    signer: &InMemorySigner,
    client: &JsonRpcClient,
    post_text: String,
) -> anyhow::Result<()> {
    // let signer = near_crypto::InMemorySigner::from_secret_key(signer_account_id, signer_secret_key);

    let access_key_query_response = client
        .call(methods::query::RpcQueryRequest {
            block_reference: BlockReference::latest(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id: signer.account_id.clone(),
                public_key: signer.public_key.clone(),
            },
        })
        .await?;

    let current_nonce = match access_key_query_response.kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKey(access_key) => {
            access_key.nonce
        }
        _ => Err(anyhow!("failed to extract current nonce"))?,
    };

    let contract_id = NEAR_SOCIAL_CONTRACT_ID.parse().unwrap();

    let transaction = Transaction {
        signer_id: signer.account_id.clone(),
        public_key: signer.public_key.clone(),
        nonce: current_nonce + 1,
        receiver_id: contract_id,
        block_hash: access_key_query_response.block_hash,
        actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
            method_name: "set".to_string(),
            args: render_post_json(signer.account_id.clone(), post_text)
                .to_string()
                .into_bytes(),
            gas: 100_000_000_000_000, // 100 TeraGas
            deposit: 0,
        }))],
    };

    let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(signer),
    };

    let tx_hash = client.call(request).await?;

    Ok(())
}

#[test]
fn test_render_post_json() {
    let account_id = "autoctopus.near".parse().unwrap();
    let rendered = render_post_json(account_id, "test".to_string()).to_string();
    dbg!(&rendered);
}
