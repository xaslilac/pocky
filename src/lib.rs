pub mod de;
pub mod md;
mod page;
mod shiki;

pub use page::html::HtmlPage;
pub use page::md::MarkdownPage;
pub use page::AsHtml;
pub use page::OrderedPageCollection;
pub use page::PageCollection;
