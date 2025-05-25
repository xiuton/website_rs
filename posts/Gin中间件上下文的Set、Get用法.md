---
title: "Gin中间件上下文的Set、Get用法"
date: "2023-01-10 00:12:05"
author: "干徒"
tags: ["Golang", "Gin"]
---

## Code

如上代码所示，中间件`middlewareA`内部通过`c.Set("middlewareA_key", str)`将参数设置到上下文中
所以中间件`middlewareB`可以通过`c.Get("middlewareA_key")`获取到上下文中传递的值，并将值进行了修改，然后继续通过上下文进行了传递
然后在控制器函数中可以通过上下文`c.Keys["middlewareB_key"]`的方式获取

```go
package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
)

func main() {
	router := gin.Default()

	router.Use(middlewareA("中间件A传递的值"), middlewareB())
	router.GET("/", func(c *gin.Context) {
		valueA := c.Keys["middlewareA_key"]
		valueB := c.Keys["middlewareB_key"]
		c.JSON(200, gin.H{
			"data": map[string]any{
				"middlewareA": valueA,
				"middlewareB": valueB,
			},
		})
	})

	if err := router.Run(":8090"); err != nil {
		fmt.Printf("run server error: %v", err)
		panic(err)
	}
}

func middlewareA(str string) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Set("middlewareA_key", str)
		c.Next()
	}
}

func middlewareB() gin.HandlerFunc {
	return func(c *gin.Context) {
		middlewareAKey, _ := c.Get("middlewareA_key")
		c.Set("middlewareB_key", middlewareAKey.(string)+" *** 我是中间件B")
		c.Next()
	}
}
```

需要注意的是，中间件设置的数据，会同时存在在控制器函数的上下文中，均可以获取

## Return
请求：http://localhost:8090
```json
{
    "data": {
        "middlewareA": "中间件A传递的值",
        "middlewareB": "中间件A传递的值 *** 我是中间件B"
    }
}
```