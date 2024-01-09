use std::{collections::{HashMap, HashSet}, iter::zip};

use chrono::NaiveDateTime;
use xlsxwriter::{format::FormatColor, Format, Workbook};

use crate::{
    validator_action::{ValidatorReceiveRewardAction, ValidatorReceiveRewardData},
    *,
};

pub async fn query_user_action() {
    let endpoint = "https://api.thegraph.com/subgraphs/name/hsxyl/near-restaking-mainnet";
    let query = r#"{
        validatorReceiveRewardActions(
			first: 100
			where: {
			  receiver: "squidward-1.near"
			  reward_uuid: null
			}
		  ) {
			receiver
			amount
			timestamp
			reward_source
			user_action {
			  receipt_id
			}
		  }
		}
  "#;

    let client = Client::new(endpoint);
    let data = client
        .query::<ValidatorReceiveRewardData>(query)
        // .query_with_vars::<ValidatorReceiveRewardData, Vars>(query, vars)
        .await
        .unwrap()
        .unwrap();

    let workbook = Workbook::new("./result.xlsx").unwrap();
    let mut sheet = workbook.add_worksheet(None).unwrap();
	let yellow_format = Format::new().set_bg_color(FormatColor::Yellow).set_font_size(20_f64).to_owned();
	let wrapped_yellow_format = Some(&yellow_format);
    sheet.write_string(0, 0, "Tx receipt", wrapped_yellow_format).unwrap();
    sheet.write_string(0, 1, "Amount", wrapped_yellow_format).unwrap();
    sheet.write_string(0, 2, "Near", wrapped_yellow_format).unwrap();
    sheet.write_string(0, 3, "Timestamp",wrapped_yellow_format).unwrap();
    sheet.write_string(0, 4, "Date", wrapped_yellow_format).unwrap();
    sheet.write_string(0, 5, "Source", wrapped_yellow_format).unwrap();
    sheet.write_string(0, 6, "is_excess", wrapped_yellow_format).unwrap();
    // let validatorReceiveRewardActions = zi
    // for i in data.validatorReceiveRewardActions.len()

	let validator_restaking_receipts: HashSet<String> = data.validatorReceiveRewardActions
	.iter()
	.filter(|e| {e.reward_source.eq("Restaking")})
	.map(|e|e.user_action.receipt_id.clone())
	.collect();

    // Format::set_font_color(&mut self, FormatColor::Black);
	let mut total_exceed_oct = 0_u128;
	let len = data.validatorReceiveRewardActions.len();
    for (i, validator_action) in zip(
        0..data.validatorReceiveRewardActions.len(),
        data.validatorReceiveRewardActions,
    ) {
        let nano_seconds = validator_action.timestamp;
        let datetime =
            NaiveDateTime::from_timestamp_millis((nano_seconds / 1000000) as i64).unwrap();
        let date = datetime.date();
        let time = datetime.time();
        let date_string = date.format("%Y-%m-%d").to_string();
        let time_string = time.format("%H:%M:%S%.9f").to_string();
		let is_excess =  !(validator_action.reward_source == "Restaking" || 
		validator_restaking_receipts.contains(&validator_action.user_action.receipt_id));
		let formater = if !is_excess {
			let mut format = Format::new();
			format.set_bg_color(FormatColor::Custom(0xF0FFF0));
			Some(format)
		} else {
			None
		};

		if is_excess {
			total_exceed_oct += validator_action.amount;
		}


        sheet
            .write_string(
                i as u32 + 1,
                0,
                validator_action.user_action.receipt_id.as_str(),
                formater.as_ref(),
            )
            .unwrap();
        sheet
            .write_string(
                i as u32 + 1,
                1,
                validator_action.amount.to_string().as_str(),
                formater.as_ref(),
            )
            .unwrap();
        sheet
            .write_string(
                i as u32 + 1,
                2,
                &convert_oct_amount_to_human_readable(validator_action.amount),
                formater.as_ref(),
            )
            .unwrap();
        sheet
            .write_string(
                i as u32 + 1,
                3,
                &validator_action.timestamp.to_string(),
                formater.as_ref(),
            )
            .unwrap();
        sheet
            .write_string(
                i as u32 + 1,
                4,
                &format!("{} {}", date_string, time_string),
                formater.as_ref(),
            )
            .unwrap();
		sheet
            .write_string(
                i as u32 + 1,
                5,
                &validator_action.reward_source,
                formater.as_ref(),
            )
            .unwrap();
		sheet.write_string(i as u32 +1 , 6, is_excess.to_string().as_str(), formater.as_ref()).unwrap();
    }
	sheet.write_string(len as u32 + 1, 0, "Total exceed oct", wrapped_yellow_format).unwrap();
	sheet.write_string(len as u32 + 1, 1, total_exceed_oct.to_string().as_str(), wrapped_yellow_format).unwrap();
	sheet.write_string(len as u32 + 1, 2, &convert_oct_amount_to_human_readable(total_exceed_oct).as_str(), wrapped_yellow_format).unwrap();

    workbook.close().unwrap();
}

#[tokio::test]
pub async fn test() {
    query_user_action().await
}
