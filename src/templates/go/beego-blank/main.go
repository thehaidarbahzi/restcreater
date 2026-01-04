package main

import (
	"fmt"

	"github.com/beego/beego/v2/server/web"
	"github.com/beego/beego/v2/server/web/context"
)

func main() {
	web.BConfig.Listen.HTTPAddr = "127.0.0.1"
	web.BConfig.Listen.HTTPPort = 8000

	web.Get("/", func(ctx *context.Context) {
		ctx.WriteString("Hello, world!")
	})

	fmt.Println("Server running at: http://127.0.0.1:8000")
	fmt.Println("Available endpoints:")
	fmt.Println("   GET / - show hello world")

	web.Run()
}
