use crate::*;

#[derive(Deserialize, Debug)]
pub struct ValidatorReceiveRewardData {
    pub validatorReceiveRewardActions: Vec<ValidatorReceiveRewardAction>,
}

#[derive(Deserialize, Debug)]
pub struct ValidatorReceiveRewardAction {
    pub receiver: String,
    #[serde(with = "u128_dec_format")]
    pub timestamp: u128,
    #[serde(with = "u128_dec_format")]
    pub amount: u128,
	pub reward_source: String,
	pub user_action: UserAction,
}

#[derive(Deserialize, Debug)]

pub struct UserAction {
    pub receipt_id: String,
}
