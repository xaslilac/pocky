use pulldown_cmark::CodeBlockKind;
use pulldown_cmark::CowStr;
use pulldown_cmark::Event;
use pulldown_cmark::Tag;
use std::io::Write;
use std::process;
use std::process::Command;

pub trait Highlight<'input, 'callback> {
	fn highlight(self) -> ShikiHighlighterParser<'input, 'callback>;
}

impl<'input, 'callback> Highlight<'input, 'callback> for pulldown_cmark::Parser<'input, 'callback> {
	fn highlight(self) -> ShikiHighlighterParser<'input, 'callback> {
		ShikiHighlighterParser {
			current_language: None,
			parser: self,
		}
	}
}

pub struct ShikiHighlighterParser<'input, 'callback> {
	current_language: Option<String>,
	parser: pulldown_cmark::Parser<'input, 'callback>,
}

impl<'input, 'callback> Iterator for ShikiHighlighterParser<'input, 'callback> {
	type Item = pulldown_cmark::Event<'input>;

	fn next(&mut self) -> Option<Self::Item> {
		let event = self.parser.next()?;

		match (&self.current_language, &event) {
			// beginning ```xxx
			(None, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))) => {
				self.current_language = Some(lang.to_string());
				Some(event)
			}
			// closing ```
			(Some(_), Event::End(Tag::CodeBlock(_))) => {
				self.current_language = None;
				Some(event)
			}
			// contained code
			(Some(lang), Event::Text(code)) => {
				Some(Event::Html(CowStr::from(highlight_code(code, lang))))
			}
			_ => Some(event),
		}
	}
}

fn highlight_code<S>(code: S, lang: &str) -> String
where
	S: AsRef<str>,
{
	let code = code.as_ref();

	let mut shiki = Command::new("node")
		.args(["-e", include_str!("./shiki.js"), lang])
		.stdin(process::Stdio::piped())
		.stdout(process::Stdio::piped())
		.spawn()
		.expect("highlighting failed");

	{
		let mut stdin = shiki.stdin.take().unwrap();

		stdin
			.write_all(code.as_bytes())
			.expect("highlighting failed");
	}

	let output = shiki.wait_with_output().expect("highlighting failed");
	let highlighted_code = String::from_utf8_lossy(&output.stdout);
	highlighted_code.to_string()
}
