use {
	chrono::{DateTime, Utc},
	itertools::Itertools,
	serde::{Deserialize, Deserializer, Serialize},
};

#[derive(Debug)]
pub enum FetchOrderError {
	Reqwest(reqwest::Error),
	Deserialization(serde_json::Error),
	NotFound,
	MultipleFound(Vec<OrderWrapper>),
}

pub fn fetch_order_by_customer_reference(reference: &str) -> Result<Order, FetchOrderError> {
	match serde_json::from_str::<Vec<OrderWrapper>>(
		&reqwest::blocking::get(&format!(
			"https://b2b.bigecommercewebsite.com/api_customer/orders?customer_order_reference_eq={reference}"
		))
		.map_err(FetchOrderError::Reqwest)?
		.text()
		.map_err(FetchOrderError::Reqwest)?,
	)
	.map_err(FetchOrderError::Deserialization)?
	.into_iter()
	.at_most_one()
	{
		Ok(Some(order_wrapper)) => Ok(order_wrapper.order),
		Ok(None) => Err(FetchOrderError::NotFound),
		Err(orders) => Err(FetchOrderError::MultipleFound(orders.collect())),
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct OrderId(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(transparent)]
pub struct OrderProductOrderId(pub i64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(transparent)]
pub struct CustomerId(pub i64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(transparent)]
pub struct ProductId(pub i64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(transparent)]
pub struct OrderProductId(pub i64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(transparent)]
pub struct StatusOrderId(pub i64);

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderWrapper {
	pub order: Order,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Order {
	pub id: OrderId,
	pub status_order_id: StatusOrderId,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub status_order_name: Option<String>,
	pub customer_id: CustomerId,
	#[serde(deserialize_with = "datetime::deserialize")]
	pub submitted_at: DateTime<Utc>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub comments_customer: Option<String>,
	pub comments_wholesaler: Option<String>,
	pub address: String,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub address2: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub city: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub province: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub country: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub postal_code: Option<String>,
	pub shipping_option_id: Option<i64>,
	#[serde(deserialize_with = "de_float")]
	pub shipping_costs: f64,
	pub shipping_tracking: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub shipping_tracking_url: Option<String>,
	pub shipping_tracking_urls_by_number: Vec<String>,
	#[serde(deserialize_with = "de_float")]
	pub total_products_before_discount: f64,
	#[serde(deserialize_with = "de_float")]
	pub discount_percentage: f64,
	pub discount_total: Option<f64>,
	#[serde(deserialize_with = "de_float")]
	pub total_products_before_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_before_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_products_after_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub gross_total: f64,
	pub extra_fields: serde_json::Value,
	pub request_delivery_at: Option<String>,
	pub integration_ref: Option<String>,
	pub order_products: Vec<OrderProductWrapper>,
	#[serde(flatten)]
	pub customer: CustomerInfo,
	pub addressbook_reference_code: Option<String>,
	pub replacement_tracking_number: Option<String>,
	pub sent_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CustomerInfo {
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_order_reference: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_email: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_company: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_name: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_phone: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub customer_reference_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderProductWrapper {
	pub order_product: OrderProduct,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct OrderProduct {
	pub id: OrderProductId,
	pub order_id: OrderProductOrderId,
	pub product_id: ProductId,
	#[serde(deserialize_with = "de_float")]
	pub quantity: f64,
	#[serde(deserialize_with = "de_float")]
	pub price: f64,
	#[serde(deserialize_with = "de_float")]
	pub final_price: f64,
	#[serde(deserialize_with = "de_float")]
	pub discount_percentage: f64,
	#[serde(deserialize_with = "de_float")]
	pub vat_percentage: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_before_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_vat: f64,
	#[serde(deserialize_with = "de_float")]
	pub total_after_vat: f64,
	pub status_order_id: Option<StatusOrderId>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub status_order_name: Option<String>,
	#[serde(default)]
	#[serde(deserialize_with = "datetime::deserialize_opt")]
	pub estimated_delivery_date: Option<DateTime<Utc>>,
	#[serde(default)]
	#[serde(deserialize_with = "datetime::deserialize_opt")]
	pub estimated_dispatch_date: Option<DateTime<Utc>>,
	pub addressbook: AddressBook,
	pub product_name: String,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub product_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AddressBook {
	pub reference_code: Option<String>,
	pub name: String,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub address: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub address2: Option<String>,
	pub city: String,
	pub province: String,
	pub country: String,
	pub postal_code: String,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub phone: Option<String>,
	#[serde(deserialize_with = "empty_string_as_none")]
	pub comments: Option<String>,
}

fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let opt = Option::<String>::deserialize(deserializer)?;
	Ok(opt.and_then(|s| if s.trim().is_empty() { None } else { Some(s) }))
}

fn de_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
	D: Deserializer<'de>,
{
	use serde::de::Error;
	let val: serde_json::Value = Deserialize::deserialize(deserializer)?;
	match val {
		serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| D::Error::custom("Invalid number")),
		serde_json::Value::String(s) => s.parse::<f64>().map_err(D::Error::custom),
		_ => Err(D::Error::custom("Expected float or string")),
	}
}

mod datetime {
	use {
		chrono::{DateTime, Utc},
		serde::{Deserialize, Deserializer},
	};

	fn parse(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
		DateTime::parse_from_rfc3339(s)
			.map(|dt| dt.with_timezone(&Utc))
			.or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z").map(|dt| dt.with_timezone(&Utc)))
	}

	pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(deserializer)?;
		parse(s).map_err(serde::de::Error::custom)
	}

	pub(super) fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let opt: Option<&str> = Option::deserialize(deserializer)?;
		match opt {
			Some(s) => Ok(Some(parse(s).map_err(serde::de::Error::custom)?)),
			None => Ok(None),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use chrono::TimeZone;

	#[test]
	fn deserialization() {
		let json_str = include_str!("orders.json");
		let deserialized: Vec<OrderWrapper> = serde_json::from_str(json_str).unwrap();

		let expected = vec![OrderWrapper {
			order: Order {
				id: OrderId("B2B111".to_string()),
				status_order_id: StatusOrderId(5),
				status_order_name: Some("Sent".to_string()),
				customer_id: CustomerId(1),
				submitted_at: Utc.with_ymd_and_hms(2019, 4, 30, 13, 35, 5).unwrap(),
				comments_customer: None,
				comments_wholesaler: None,
				address: "Teststraat 10".to_string(),
				address2: None,
				city: Some("Rotterdam".to_string()),
				province: None,
				country: Some("NL".to_string()),
				postal_code: Some("9101AA".to_string()),
				shipping_option_id: None,
				shipping_costs: 0.0,
				shipping_tracking: None,
				shipping_tracking_url: None,
				shipping_tracking_urls_by_number: vec![],
				total_products_before_discount: 2.0,
				discount_percentage: 0.0,
				discount_total: None,
				total_products_before_vat: 2.0,
				total_before_vat: 2.0,
				total_vat: 0.0,
				total_products_after_vat: 2.154,
				gross_total: 3.6648,
				extra_fields: serde_json::json!({}),
				request_delivery_at: None,
				integration_ref: None,
				order_products: vec![OrderProductWrapper {
					order_product: OrderProduct {
						id: OrderProductId(1234),
						order_id: OrderProductOrderId(123),
						product_id: ProductId(9999),
						quantity: 1.0,
						price: 2.0,
						final_price: 2.0,
						discount_percentage: 0.0,
						vat_percentage: 7.7,
						total_before_vat: 2.0,
						total_vat: 0.154,
						total_after_vat: 2.154,
						status_order_id: None,
						status_order_name: None,
						estimated_delivery_date: None,
						estimated_dispatch_date: None,
						addressbook: AddressBook {
							reference_code: None,
							name: "test".to_string(),
							address: Some("test".to_string()),
							address2: None,
							city: "tes".to_string(),
							province: "tes".to_string(),
							country: "NL".to_string(),
							postal_code: "1234".to_string(),
							phone: None,
							comments: None,
						},
						product_name: "Test product".to_string(),
						product_code: Some("123456".to_string()),
					},
				}],
				customer: CustomerInfo {
					customer_order_reference: Some("99991111".to_string()),
					customer_email: Some("examplemail@gmail.com".to_string()),
					customer_company: Some("Example-company".to_string()),
					customer_name: Some("Example name".to_string()),
					customer_phone: Some("1234568129".to_string()),
					customer_reference_code: None,
				},
				addressbook_reference_code: None,
				replacement_tracking_number: Some("**************".to_string()),
				sent_date: None,
			},
		}];

		assert_eq!(deserialized, expected);
	}
}
