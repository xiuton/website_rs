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
    /// 所属系列名（多章节文档），空串表示独立文章
    pub series: &'static str,
    /// 章节在系列中的顺序，缺省为 0
    pub order: i32,
}

