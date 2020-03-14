use std::error::Error;
use reqwest;

pub use config::{Config, ConfigErr};
pub use article::{Article, ArticleErr};
pub use url::{URL, URLErr};

pub mod article;
pub mod url;
pub mod config;

/// The main function of this library. Running this allows you to find a
/// graph around a certain set of Wikipedia articles and possibly the shortest
/// paths between them.
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Implement running logic.
    let resp = reqwest::blocking::get("https://en.wikipedia.org/wiki/Pragma_once")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}
