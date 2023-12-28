use near_gas::NearGas;

use crate::*;

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
                to_human(user_action.accept_offering_action.unwrap().payment_balance)
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

pub async fn send_post(signer: &Account, post_text: String) -> ExecutionFinalResult {
    let contract_id = NEAR_SOCIAL_CONTRACT_ID.parse().unwrap();
    signer
        .call(&contract_id, "set")
        .args_json(render_post_json(signer.id().clone(), post_text))
        .gas(NearGas::from_tgas(50))
        .transact()
        .await
        .unwrap()
}

#[test]
fn test_render_post_json() {
    let account_id = "autoctopus.near".parse().unwrap();
    let rendered = render_post_json(account_id, "test".to_string()).to_string();
    dbg!(&rendered);
}
