---
title: "Taro快速体验"
date: "2023-01-12 09:30:08"
author: "干徒"
tags: ["Taro"]
---

# 5分钟快速入门Taro

## Taro介绍

Taro是由凹凸实验室开发的一个开放式跨端跨框架，支持使用 React/Vue/Nerv 等框架来开发 [微信](https://mp.weixin.qq.com/) / [京东](https://mp.jd.com/?entrance=taro) / [百度](https://smartprogram.baidu.com/) / [支付宝](https://mini.open.alipay.com/) / [字节跳动](https://developer.open-douyin.com/) / [QQ](https://q.qq.com/) / [飞书](https://open.feishu.cn/document/uYjL24iN/ucDOzYjL3gzM24yN4MjN) 小程序 / H5 / RN 等应用

Taro类似于uniapp，都是跨端框架，一次开发多端部署

这是[Taro官方文档](https://taro-docs.jd.com/)，可以结合官方文档食用更佳

## 准备

你需要了解最基本的 HTML + CSS + JS ，俗称前端三件套

最好会使用Vue React

node环境（>=16.20.0）



## 安装Taro CLI工具

安装

```sh
npm install -g @tarojs/cli
```

查看是否安装成功

```sh
npm info @tarojs/cli
```



## 初始化项目

```sh
taro init myApp
```

![taro init myApp command screenshot](./Taro快速入门.assets/ecb98df1436cd3d5.jpg)

进入到项目根目录，安装依赖

```sh
cd myApp

pnpm i
```



## 启动项目

开发

```sh
pnpm dev:weapp
```



打包

```sh
pnpm build:weapp
```



项目启动之后，在微信开发者工具中打开该项目根目录，即可预览项目

这样Taro项目就启动起来了