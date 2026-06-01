---
title: "Vue3.3+ 新特性 defineOptions"
date: 2023-12-21 00:45:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17917868.html
tags: ["JavaScript", "Vue", "前端", "技术"]
---

# Vue3.3+ 新特性 defineOptions

`defineOptions`是一个宏，是在Vue3.3+中新增的新特性

# defineOptions配置项

## name

在Vue3.3之前，组件的默认组件名为.vue单文件组件\[SFC\]文件的名字，如果需要修改组件名则需要结合Options API进行配置

```vue
<!-- src/components/Com.vue -->
<script setup>

</script>

<script>
export default {
  name: 'ComponentName'
}
</script>

<template>
  <div>Com Component</div>
</template>

```

在以上的代码中，写了两个script标签，一个标签写Composition API代码，一个标签以Options API的方式配置组件名  
虽然可以配置组件名，但是这种处理方式很不好

在Vue3.3+新增的defineOptions宏，可以很好的解决这个问题

```vue
<!-- src/components/Com.vue -->
<script setup>
defineOptions({
  name: 'ComponentName'
})
</script>

<template>
  <div>Com Component</div>
</template>

```

在以上代码中可以看到，defineOptions是全局的宏，无需导入

## inheritAttrs

在Vue中如果在组件标签上添加属性，则会被透传到组件的根标签上，*在Vue3中，组件的根标签可以存在多个，如果根标签存在多个，那么透传将失效*

```vue
<!-- src/App.vue -->
<script setup>
import Com from './components/Com.vue'
</script>

<template>
  <Com class="comComponentClassName" />
</template>

```

```vue
<!-- src/components/Com.vue -->
<script setup>
defineOptions({
  name: 'ComponentName',
})
</script>

<template>
  <div>Com Component</div>
</template>

```

解析后的代码

```html
<div id="app" data-v-app="">
  <div class="comComponentClassName">Com Component</div>
</div>

```

有时在开发中，并不希望组件存在透传行为，我们可以通过配置进行关闭

```vue
<!-- src/components/Com.vue -->
<script setup>
defineOptions({
  name: 'ComponentName',
})
</script>

<script>
export default {
  inheritAttrs: false
}
</script>

<template>
  <div>Com Component</div>
</template>

```

以上代码，在Vue3.3之前，如需配置组件禁用透传，那么配置方式和配置组件名一样，需要在单独的script写Options API配置项

在Vue3.3+可以通过defineOptions宏进行配置

```vue
<!-- src/components/Com.vue -->
<script setup>
defineOptions({
  name: 'ComponentName',
  inheritAttrs: false,
})
</script>

<template>
  <div>Com Component</div>
</template>

```

`defineOptions`宏还可以配置其他内容，具体内容请查看官方文档 [https://cn.vuejs.org/api/sfc-script-setup.html#defineoptions](https://cn.vuejs.org/api/sfc-script-setup.html#defineoptions)