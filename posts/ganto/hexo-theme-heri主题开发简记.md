---
title: "hexo-theme-heri主题开发简记"
date: 2021-01-05 15:59:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/14236348.html
---

# hexo-theme-heri主题开发简记

# Heri介绍：

究极简洁的Hexo主题  
![预览图](https://heri.ganto.cn/ArticleResources/Hexo-Theme-Heri/heri.png)

## 展示站点

[展示站点](https://heri.ganto.cn)

# 下载Heri

[hexo-theme-heri](https://github.com/ganto-cn/hexo-theme-heri/releases)

下载完整的Heri主题，拷贝到Hexo的主题文件夹下

# 引用Heri

在Hexo根配置文件中这样引用即可

```makefile
theme: heri

```

# 根配置文件的其他必要配置

```yaml
# Site
title: Ganto # 网站名称
subtitle: '但愿可以成为有趣的人' # 网站小标题，可为座右铭
# URL
url: https://www.baidu.cn # 网站链接
root: / # 如无特殊，设置"/"即可

```

# 主题配置文件的必要配置

```yaml
# 网站icon
favicon: /images/g.png # 网站的icon

# 日期/时间格式
time_format:
  date_format: YYYY-MM-DD
  time_format: HH:mm:ss
  division: "" # 日期与时间中间的间隔，如：division: "/" ===>>> 2020-20-20/20:20:20

# 网站声明
website_notice:
  txt: 如果你需要“转载”、“引用”小站的文章，可以不需要作者同意，请务必标明出处和文章链接。 # 文章末尾处的声明文字

```

# 代码高亮

如需代码高亮，请这样设置根配置文件

```vbnet
hljs: true

```

# 图片预览插件

[fancybox](https://fancyapps.com/)  
![预览图](https://heri.ganto.cn/ArticleResources/Hexo-Theme-Heri/0.jpg)

# 动效插件

[Animate.css](https://animate.style/)

# 代码高亮

[highlight](https://highlightjs.org/)  
![预览图](https://heri.ganto.cn/ArticleResources/Hexo-Theme-Heri/1.png)

目及所见，皆是全部；化繁为简，只为高效。

没错，这就是Heri的全部。