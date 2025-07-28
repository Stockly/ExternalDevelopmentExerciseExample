use serde::{Deserialize, Deserializer};

pub(crate) fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let opt = Option::<String>::deserialize(deserializer)?;
	Ok(opt.and_then(|s| if s.trim().is_empty() { None } else { Some(s) }))
}

pub(crate) fn de_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
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

pub(crate) mod datetime {
	use {
		chrono::{DateTime, Utc},
		serde::{Deserialize, Deserializer},
	};

	fn parse(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
		DateTime::parse_from_rfc3339(s)
			.map(|dt| dt.with_timezone(&Utc))
			.or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z").map(|dt| dt.with_timezone(&Utc)))
	}

	pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(deserializer)?;
		parse(s).map_err(serde::de::Error::custom)
	}

	pub(crate) fn deserialize_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
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
