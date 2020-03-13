use super::url::URL;
use std::error::Error;
use std::fs;
use thiserror::Error;
/// ConfigErr is an enum that contains possible error values that
/// could occur during the Configuration of this library in Config::new.
#[derive(Error, Debug)]
pub enum ConfigErr {
    //#[error("Found too many arguments. (expected: 2)")]
    //TooManyArguments,
    #[error("Found too few arguments. (expected: 2)")]
    TooFewArguments,
    #[error("Could not parse search depth. (found {0})")]
    IntParseError(String),
    #[error("Found no valid urls in the file.")]
    NoValidUrls,
}
/// Config is a struct used to bundle all the possible configurations
/// for the wikigraph library.
pub struct Config {
    /// Contains a list of URLs to valid Wikipedia articles.
    urls: Vec<URL>,
    /// Contains the depth for the search in the Wikipedia graph.
    depth: u32,
}

impl Config {
    /// Given an array of (command-line) arguments this function creates a fitting
    /// configurations file.
    ///
    /// Excatly two arguments are expected, otherwise an error is returned.
    /// - An integer containing the desired search depth.
    /// - A file name containing the starting URLs.
    pub fn new(mut args: std::env::Args) -> Result<Config, Box<dyn Error>> {
        // Dropping the name of the executable.
        args.next();
        // Parsing the depth.
        let n = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(v) => v,
                Err(_) => return Err(Box::new(ConfigErr::IntParseError(arg))),
            },
            None => return Err(Box::new(ConfigErr::TooFewArguments)),
        };
        // Parsing the URL file
        let urls = match args.next() {
            Some(arg) => Config::get_urls(&arg),
            None => return Err(Box::new(ConfigErr::TooFewArguments)),
        };
        match urls {
            Err(e) => Err(e),
            Ok(v) => Ok(Config { urls: v, depth: n }),
        }
    }

    /// Filters all the valid Wikipedia articles from a given String.
    /// Articles have to be on separate lines and follow the criteria specified in the scraper module.
    fn get_urls(path: &String) -> Result<Vec<URL>, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let lines = contents.lines();
        let valid_urls = URL::new_list(lines);
        if valid_urls.len() == 0 {
            return Err(Box::new(ConfigErr::NoValidUrls));
        }
        return Ok(valid_urls);
    }
}
