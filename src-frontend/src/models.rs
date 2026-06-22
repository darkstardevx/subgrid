use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub timestamp: String,
    pub link: String,
}

// ... include previous models like GithubRepo and AurPackage
