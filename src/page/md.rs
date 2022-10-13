use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::md::markdown_to_html;
use crate::AsHtml;

#[derive(Clone, Debug)]
pub struct MarkdownPage<M: DeserializeOwned> {
	pub metadata: Option<M>,
	pub content: String,
}

impl<M: DeserializeOwned> MarkdownPage<M> {
	pub fn parse(content: String) -> Self {
		// Skip blank lines from the beginning
		let mut lines = content
			.lines()
			.skip_while(|line| line.trim().is_empty())
			.peekable();

		let is_frontmatter_delimiter =
			|line: &&str| line.len() >= 3 && line.find(|c| c != '-').is_none();

		// Parse the frontmatter section, if the document starts with one
		let metadata = lines.next_if(is_frontmatter_delimiter).map(|_| {
			let metadata_source = lines
				.by_ref()
				.take_while(|line| !is_frontmatter_delimiter(line))
				.map(|line| format!("{}\n", line))
				.collect::<String>();

			serde_yaml::from_str(&metadata_source)
				.expect("failed to deserialize frontmatter metadata")
		});

		// The remaining lines are the actual document content
		let md_source = lines.map(|line| format!("{}\n", line)).collect::<String>();
		let content = markdown_to_html(md_source);

		MarkdownPage { metadata, content }
	}
}

impl<M, P> From<P> for MarkdownPage<M>
where
	P: AsRef<Path>,
	M: DeserializeOwned,
{
	fn from(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		MarkdownPage::parse(content)
	}
}

impl<M> AsHtml for MarkdownPage<M>
where
	M: DeserializeOwned,
{
	fn as_html(&self) -> String {
		self.content.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	type BasicMarkdownPage = MarkdownPage<HashMap<String, String>>;

	use serde::Deserialize;
	use std::collections::HashMap;

	#[test]
	fn no_frontmatter() {
		let page = BasicMarkdownPage::parse("# Hello, friend!\n".to_string());

		assert_eq!(page.metadata, None);
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}

	#[test]
	fn with_frontmatter() {
		let page = BasicMarkdownPage::parse(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(
			page.metadata.unwrap().get("title"),
			Some(&"Cool video games".to_string())
		);
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}

	#[test]
	fn with_frontmatter_deserialize() {
		#[derive(Deserialize)]
		struct TitleMetadata {
			pub title: String,
		}

		let page = MarkdownPage::<TitleMetadata>::parse(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(page.metadata.unwrap().title, "Cool video games".to_string());
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}
}
