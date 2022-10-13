use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct MarkdownPage {
	pub metadata: HashMap<String, String>,
	pub content: String,
}

impl MarkdownPage {
	pub fn parse(content: String) -> Self {
		// Skip blank lines from the beginning
		let mut lines = content
			.lines()
			.skip_while(|line| line.trim().is_empty())
			.peekable();

		let is_frontmatter_delimiter =
			|line: &&str| line.len() >= 3 && line.find(|c| c != '-').is_none();

		// Parse the frontmatter section, if the document starts with one
		let metadata = if lines.next_if(is_frontmatter_delimiter).is_none() {
			HashMap::<String, String>::default()
		} else {
			let metadata_source = lines
				.by_ref()
				.take_while(|line| !is_frontmatter_delimiter(line))
				.map(|line| format!("{}\n", line))
				.collect::<String>();

			serde_yaml::from_str(&metadata_source).unwrap()
		};

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

impl<P> From<P> for MarkdownPage
where
	P: AsRef<Path>,
{
	fn from(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		MarkdownPage::parse(content)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_frontmatter() {
		let page = MarkdownPage::parse("# Hello, friend!\n".to_string());

		assert_eq!(page.metadata.len(), 0);
		assert_eq!(
			page.content,
			"<h1 id='hello,_friend!'>Hello, friend!</h1>\n"
		);
	}
	#[test]
	fn with_frontmatter() {
		let page = MarkdownPage::parse(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(
			page.metadata.get("title"),
			Some(&"Cool video games".to_string())
		);
		assert_eq!(
			page.content,
			"<h1 id='hello,_friend!'>Hello, friend!</h1>\n"
		);
	}
}
