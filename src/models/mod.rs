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

