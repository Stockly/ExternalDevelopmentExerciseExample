mod deser;
mod structs;
#[cfg(test)]
mod tests;

use structs::*;

use itertools::Itertools;

pub fn fetch_order_by_customer_reference(reference: &str) -> Result<Order, FetchOrderError> {
	match serde_json::from_str::<Vec<OrderWrapper>>(
		&reqwest::blocking::get(&format!(
			"https://b2b.bigecommercewebsite.com/api_customer/orders?customer_order_reference_eq={reference}"
		))?
		.text()?,
	)?
	.into_iter()
	.at_most_one()
	{
		Ok(Some(order_wrapper)) => Ok(order_wrapper.order),
		Ok(None) => Err(FetchOrderError::NotFound),
		Err(orders) => Err(FetchOrderError::MultipleFound(orders.collect())),
	}
}
