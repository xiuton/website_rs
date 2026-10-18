//! 给站点数据生成器打一个“源码指纹”。
//!
//! `generate_all` 用它 + 文章内容 hash 判断能否复用上一轮生成结果；
//! 生成器实现变了指纹就变，避免缓存到旧格式。

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

fn hash_dir(dir: &Path, hasher: &mut DefaultHasher) {
    let mut entries: Vec<_> = match fs::read_dir(dir) {
        Ok(entries) => entries.flatten().collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            hash_dir(&path, hasher);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            path.hash(hasher);
            if let Ok(content) = fs::read(&path) {
                content.hash(hasher);
            }
        }
    }
}

fn main() {
    let mut hasher = DefaultHasher::new();
    hash_dir(Path::new("src"), &mut hasher);
    println!("cargo:rustc-env=SITE_DATA_HASH={:x}", hasher.finish());
    println!("cargo:rerun-if-changed=src");
}
