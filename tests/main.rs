use pocky::AsHtml;
use pocky::MarkdownPage;
use std::collections::HashMap;
use std::fs;
use std::io;

#[test]
fn generate_html_for_sample_blog_post() -> io::Result<()> {
	let post = MarkdownPage::<HashMap<String, String>>::from("./tests/testdata/sample.md");
	fs::write("./tests/testdata/sample.html", post.as_html())?;

	Ok(())
}
