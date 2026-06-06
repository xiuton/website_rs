#[derive(Clone, PartialEq, Debug)]
pub struct BlogPost {
    pub title: &'static str,
    pub date: &'static str,
    pub author: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
    pub slug: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RuntimeBlogPost {
    pub title: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub content: String,
    pub slug: String,
    pub category: String,
    pub summary: String,
}

impl RuntimeBlogPost {
    pub fn from_static(post: &BlogPost) -> Self {
        RuntimeBlogPost {
            title: post.title.to_string(),
            date: post.date.to_string(),
            author: post.author.to_string(),
            tags: post.tags.iter().map(|&s| s.to_string()).collect(),
            content: post.content.to_string(),
            slug: post.slug.to_string(),
            category: post.category.to_string(),
            summary: post.summary.to_string(),
        }
    }
}
