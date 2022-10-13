pub mod md;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::vec;

pub trait AsHtml {
	fn as_html(&self) -> String;
}

#[derive(Clone, Debug, Default)]
pub struct PageCollection<E: AsHtml> {
	pages: Vec<E>,
}

impl<E> PageCollection<E>
where
	E: AsHtml,
{
	pub fn iter(&self) -> impl Iterator<Item = &E> {
		self.pages.iter()
	}

	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(&E) -> bool,
	{
		self.pages.retain(f);
	}
}

impl<E, P> From<P> for PageCollection<E>
where
	E: AsHtml + From<PathBuf> + Ord + PartialOrd,
	P: AsRef<Path>,
{
	fn from(path: P) -> Self {
		let mut pages = fs::read_dir(path)
			.expect("could not read directory contents")
			.flatten()
			.filter(|entry| {
				entry
					.file_type()
					.map(|file_type| file_type.is_file())
					.unwrap_or(false)
			})
			.map(|entry| E::from(entry.path()))
			.collect::<Vec<_>>();

		pages.sort();

		Self { pages }
	}
}

impl<E> IntoIterator for PageCollection<E>
where
	E: AsHtml,
{
	type Item = E;
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.pages.into_iter()
	}
}
