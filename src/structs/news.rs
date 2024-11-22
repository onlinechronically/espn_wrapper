use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct News {
    pub header: String,
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub description: String,
    pub headline: String,
    pub byline: Option<String>,
    pub links: ArticleLinkCollection,
}

#[derive(Debug, Deserialize)]
pub struct ArticleLinkCollection {
    pub web: ArticleLink,
}

#[derive(Debug, Deserialize)]
pub struct ArticleLink {
    pub href: String,
}
