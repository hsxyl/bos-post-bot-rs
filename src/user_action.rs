use near_primitives::types::AccountId;

use crate::*;

#[derive(Deserialize, Debug)]
pub struct Listing {
	seller_id: String,
	#[serde(with = "u128_dec_format")]
	price: u128,
}

#[derive(Deserialize, Debug)]
pub enum ActionType {
	// market
	create_listing_action,
	update_listing_action,
	create_offering_action,
	update_offering_action,
	buy_listing_action,
	accept_offering_action,

	// nft
	nft_mint_action,
}

impl ActionType {
	fn post_text(&self, user_action: UserAction) {

		match self {
			ActionType::create_listing_action => todo!(),
    		ActionType::update_listing_action => todo!(),
    		ActionType::create_offering_action => todo!(),
    		ActionType::update_offering_action => todo!(),
    		ActionType::buy_listing_action => todo!(),
    		ActionType::accept_offering_action => todo!(),
    		ActionType::nft_mint_action => todo!()
		}


	}
	
}



#[derive(Deserialize, Debug)]

pub struct UserAction {
	pub id: String,
	pub user_id: String,
	pub timestamp: String,
	pub receipt_id: String,

	pub contract_id: String,
	pub token_id: String,

	pub action_type: ActionType, 

	pub create_listing_action: Option<CreateListingAction>, 
	pub update_listing_action: Option<UpdateListingAction>, 
	pub create_offering_action: Option<CreateOfferingAction>,
	pub update_offering_action: Option<UpdateOfferingAction>,
	pub nft_mint_action: Option<NftMintAction>,

	pub buy_listing_action: Option<BuyListingAction>,
	pub accept_offering_action: Option<AcceptOfferingAction>,

}

#[derive(Deserialize, Debug)]
pub struct BuyListingAction {
	pub buyer_id: AccountId,
	pub seller_id: AccountId,
	#[serde(with = "u128_dec_format")]
	pub payment_balance: u128,
	#[serde(with = "u128_dec_format")]
	pub payout_balance: u128,
}

#[derive(Deserialize, Debug)]
pub struct AcceptOfferingAction {
	pub buyer_id: String,
	pub seller_id: String,
	#[serde(with = "u128_dec_format")]
	pub payment_balance: u128,
	#[serde(with = "u128_dec_format")]
	pub payout_balance: u128
}

#[derive(Deserialize, Debug)]
pub struct CreateListingAction {
	#[serde(with = "u128_dec_format")]
	pub price: u128,
	pub listing: Listing,
}

#[derive(Deserialize, Debug)]
pub struct UpdateListingAction {
	#[serde(with = "u128_dec_format")]
	pub old_price: u128,
	#[serde(with = "u128_dec_format")]
	pub new_price: u128,
	pub listing: Listing,
}

#[derive(Deserialize, Debug)]
pub struct CreateOfferingAction {
	pub offer_creator: String,
	#[serde(with = "u128_dec_format")]
	pub price: u128,
	pub offering_id: String
}

#[derive(Deserialize, Debug)]
pub struct UpdateOfferingAction {
	pub offer_creator: String,
	#[serde(with = "u128_dec_format")]
	pub old_price: u128,
	#[serde(with = "u128_dec_format")]
	pub new_price: u128,
	pub offering_id: String
}

#[derive(Deserialize, Debug)]
pub struct NftMintAction {
	pub owner_id: String,
}
