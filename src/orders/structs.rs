use {
	chrono::{DateTime, Utc},
	serde::{Deserialize, Serialize},
};

use super::deser::{datetime, de_float, empty_string_as_none};

#[derive(Debug, derive_more::From)]
pub enum FetchOrderError {
	Reqwest(reqwest::Error),
	Deserialization(serde_json::Error),
	NotFound,
	MultipleFound(Vec<OrderWrapper>),
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
