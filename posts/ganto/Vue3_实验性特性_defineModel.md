---
title: "Vue3 实验性特性 defineModel"
date: 2023-12-21 00:18:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/17917902.html
tags: ["JavaScript", "Vue", "前端", "技术"]
---

# Vue3 实验性特性 defineModel

> 在Vue3.4中，defineModel宏已经被正式启用，无需配置，直接可用

# 启用defineModel

目前\[Vue3.3.13\]defineModel宏是实验性特性，如果需要使用该宏，需要在`vite.config.js`配置以下内容

```js
export default defineConfig({
  plugins: [
    vue({
      script: {
        defineModel: true
      }
    }),
  ],
})

```

因为是实验性功能，在运行项目时，会有提示

> \[@vue/compiler-sfc\] defineModel() is an experimental feature and disabled by default.  
> To enable it, follow the RFC at [https://github.com/vuejs/rfcs/discussions/503](https://github.com/vuejs/rfcs/discussions/503).

# defineModel出现的必要

在Vue中，父传子的数据是单向数据流，即无法直接通过子组件修改父组件传递过来的数据  
如果需要修改可以通过自定义事件的方式，通过子组件的自定义事件来修改父组件的状态

```vue
<!-- src/App.vue -->
<script setup>
import { ref } from 'vue'
import Counter from './components/Counter.vue'

const count = ref(0)
const updateCount = (val) => {
  count.value = val
}
</script>

<template>
  <div>父组件：{{ count }}</div>
  <Counter :count="count" @addCount="updateCount"/>
</template>

```

```vue
<!-- src/components/Counter.vue -->
<script setup>
const props = defineProps({
  count: Number
})

const emit = defineEmits(["addCount"])

const handleClick = () => {
  emit("addCount", props.count+1)
}
</script>

<template>
  <div>子组件：{{ count }}</div>
  <button @click="handleClick">count+1</button>
</template>

```

以上代码可以实现子组件修改父组件的状态，虽然不算复杂，也很好理解原理，但是这并不优雅

可以通过v-model来进一步简化代码，以下代码父组件并不是通过属性的方式将数据传递给子组件，而是通过v-model进行传递  
子组件的代码量基本和上边的代码一致，只是自定义事件名变成了`update:count`的形式，这是固定写法  
父组件无需注册子组件的自定义事件

```vue
<!-- src/App.vue -->
<script setup>
import { ref } from 'vue'
import Counter from './components/Counter.vue'
const count = ref(0)
</script>

<template>
  <div>父组件：{{ count }}</div>
  <Counter v-model:count="count"/>
</template>

```

> 如果父组件直接写`v-model="count"`，其实可以理解为默认转换为`v-model:modelValue="count"`，即：`:modelValue="count"` `@update:modelValue="count"`  
> 子组件中需要props接收`modelValue`，自定义事件将变为`update:modelValue`

```vue
<!-- src/components/Counter.vue -->
<script setup>
const props = defineProps({
  count: Number
})

const emit = defineEmits(["update:count"])

const handleClick = () => {
  emit("update:count", props.count+1)
}
</script>

<template>
  <div>子组件：{{ count }}</div>
  <button @click="handleClick">count+1</button>
</template>

```

虽然简化了父组件的代码，但是子组件的代码量并没有减少，并且子组件自定义事件名变成了固定格式的写法  
这不仅让人产生疑惑: "自定义事件没有在父组件中被使用，这无疑是增加代码阅读的成本"

这些疑惑可以通过`defineModel`宏来打消  
父组件还是一样的写法，只需子组件通过defineModel宏来处理，甚至不用defineProps来定义父组件传递的数据

```vue
<!-- src/App.vue -->
<script setup>
import { ref } from 'vue'
import Counter from './components/Counter.vue'

const count = ref(0)
</script>

<template>
  <div>父组件：{{ count }}</div>
  <Counter v-model:count="count"/>
</template>

```

```vue
<!-- src/components/Counter.vue -->
<script setup>
const count = defineModel('count')

const handleClick = () => {
  count.value += 1
}
</script>

<template>
  <div>子组件：{{ count }}</div>
  <button @click="handleClick">count+1</button>
</template>

```

defineModel宏目前还是实验性功能，不应该在生产环境中进行使用