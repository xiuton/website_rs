//! 按站点实际用到的字符裁剪字体，输出 woff2 到 `static/fonts/`。
//!
//! 纯 Rust 实现（fontcull / klippa + woofwoof），不再依赖 Python 与 fonttools：
//!
//! ```bash
//! cargo run -p font-subset
//! ```
//!
//! 为什么要裁剪：原始字体合计约 10.4 MB，其中绝大多数字形永远不会被用到，
//! 而字体是首屏 LCP 的主要瓶颈。裁剪后三套字体合计约 0.5 MB。
//!
//! 源字体放在 `assets/fonts/`（不参与部署）。新增文章后如果出现缺字
//! （浏览器回退到系统字体），重跑本工具即可。

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use allsorts::font_data::FontData;

/// 所有字体都保留的基础字符：ASCII 可打印字符 + 中文常用标点。
const BASE_CHARS: &str = "，。！？：；、“”‘’（）【】《》—…·　「」『』";

const TEXT_EXTENSIONS: &[&str] = &["md", "rs", "scss", "css", "html", "toml", "txt"];

/// 递归扫描时需要跳过的目录（构建产物、依赖、字体源文件）。
const SKIP_DIRS: &[&str] = &["target", "dist", "node_modules", ".git", "assets", "static"];

const FONTS: &[(&str, &str)] = &[
    ("MiSans-Regular.otf", "MiSans-subset.woff2"),
    ("JetBrainsMonoNLNerdFont-Regular.ttf", "JetBrainsMono-subset.woff2"),
    ("SmileySans-Oblique.otf", "SmileySans-subset.woff2"),
];

fn main() -> Result<(), Box<dyn Error>> {
    let root = project_root()?;
    let source_dir = root.join("assets").join("fonts");
    let output_dir = root.join("static").join("fonts");
    fs::create_dir_all(&output_dir)?;

    let mut chars: HashSet<char> = BASE_CHARS.chars().collect();
    chars.extend((0x20u32..=0x7E).filter_map(char::from_u32));

    let mut scanned = 0usize;
    for dir in ["posts", "src"] {
        scanned += collect_from_dir(&root.join(dir), &mut chars)?;
    }
    for file in ["index.html", "data/bookmarks.toml", "README.md"] {
        if collect_from_file(&root.join(file), &mut chars)? {
            scanned += 1;
        }
    }

    println!("collected {} unique characters from {} files", chars.len(), scanned);

    let codepoints: BTreeSet<char> = chars.iter().copied().collect();

    for (input, output) in FONTS {
        let input_path = source_dir.join(input);
        let output_path = output_dir.join(output);
        let data = fs::read(&input_path).map_err(|e| format!("读取 {} 失败: {e}", input_path.display()))?;

        let scope = allsorts::binary::read::ReadScope::new(&data);
        let font_data = scope
            .read::<FontData>()
            .map_err(|e| format!("解析 {} 失败: {e}", input_path.display()))?;
        let provider = font_data
            .table_provider(0)
            .map_err(|e| format!("读取 {} 表失败: {e}", input_path.display()))?;

        let face =
            ttf_parser::Face::parse(&data, 0).map_err(|e| format!("读取 {} 字形表失败: {e}", input_path.display()))?;

        // .notdef 必须保留，其余只保留站点实际用到的字符；glyph id 不能重复。
        let mut glyph_ids: Vec<u16> = codepoints
            .iter()
            .filter_map(|c| face.glyph_index(*c))
            .map(|g| g.0)
            .filter(|g| *g != 0)
            .collect();
        glyph_ids.sort_unstable();
        glyph_ids.dedup();
        glyph_ids.insert(0, 0);

        let subset = allsorts::subset::subset(
            &provider,
            &glyph_ids,
            &allsorts::subset::SubsetProfile::Minimal,
            allsorts::subset::CmapTarget::Unicode,
        )
        .map_err(|e| format!("裁剪 {} 失败: {e}", input_path.display()))?;

        let glyphs = |bytes: &[u8]| {
            ttf_parser::Face::parse(bytes, 0)
                .map(|face| face.number_of_glyphs())
                .unwrap_or(0)
        };
        println!(
            "  glyphs {} -> {}, sfnt {} KB -> {} KB",
            glyphs(&data),
            glyphs(&subset),
            data.len() / 1024,
            subset.len() / 1024
        );

        // CFF（OTF）与 TrueType 都交给 woff2 的同一套编码路径；
        // `transform = true` 让 TrueType 使用 glyf/loca 变换，CFF 走 null transform。
        let woff2 =
            woofwoof::compress(&subset, "", 8, true).ok_or_else(|| format!("压缩 {} 失败", input_path.display()))?;

        // 自检：能解压回 SFNT 才写入，避免产出损坏的字体文件
        if woofwoof::decompress(&woff2).is_none() {
            return Err(format!("{} 生成的 woff2 无法解压", output_path.display()).into());
        }

        fs::write(&output_path, &woff2).map_err(|e| format!("写入 {} 失败: {e}", output_path.display()))?;

        println!("{:<45} -> {:<34} {:>8.1} KB", input, output, woff2.len() as f64 / 1024.0);
    }

    Ok(())
}

/// 项目根目录：`tools/font-subset` 的上两级。
fn project_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "无法定位项目根目录".into())
}

fn collect_from_dir(dir: &Path, chars: &mut HashSet<char>) -> Result<usize, Box<dyn Error>> {
    if !dir.is_dir() {
        return Ok(0);
    }

    let mut count = 0usize;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if !SKIP_DIRS.contains(&name.as_ref()) {
                    stack.push(path);
                }
            } else if collect_from_file(&path, chars)? {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// 读取文本文件并收集字符；非文本扩展名返回 `false`。
fn collect_from_file(path: &Path, chars: &mut HashSet<char>) -> Result<bool, Box<dyn Error>> {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return Ok(false);
    };
    if !TEXT_EXTENSIONS.contains(&ext) {
        return Ok(false);
    }

    let text = fs::read_to_string(path)?;
    chars.extend(text.chars().filter(|c| !c.is_control()));
    Ok(true)
}
