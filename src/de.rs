use serde::Deserialize;
use serde::Deserializer;

use crate::md::markdown_to_html;

pub fn comma_separated<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let tags = String::deserialize(de)?
		.split(',')
		.map(|tag| tag.trim().to_string())
		.collect();

	Ok(tags)
}

pub fn markdown<'de, D>(de: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(markdown_to_html(String::deserialize(de)?))
}

pub fn option_markdown<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Some(markdown_to_html(String::deserialize(de)?)))
}
