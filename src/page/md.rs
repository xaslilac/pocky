use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

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

		let options = {
			use pulldown_cmark::Options;

			let mut options = Options::empty();
			options.insert(Options::ENABLE_STRIKETHROUGH);
			options.insert(Options::ENABLE_TABLES);
			options.insert(Options::ENABLE_TASKLISTS);

			options
		};

		// Allocate *roughly* enough room. It'll probably resize at least once, but this
		// should prevent it from needing to resize multiple times.
		let mut content = String::with_capacity(md_source.len());
		let parser = pulldown_cmark::Parser::new_ext(&md_source, options);
		pulldown_cmark::html::push_html(&mut content, parser);

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
