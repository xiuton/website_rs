#[derive(Clone, PartialEq)]
pub struct BlogPost {
    pub title: &'static str,
    pub date: &'static str,
    pub author: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
    pub slug: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct RuntimeBlogPost {
    pub title: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub content: String,
    pub slug: String,
} 