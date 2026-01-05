package main

import (
	"fmt"
	"log"

	"github.com/gobuffalo/buffalo"
)

func main() {
	fmt.Println("Server running at: http://127.0.0.1:8000")
	fmt.Println("Available endpoints:")
	fmt.Println("   GET / - show hello world")

	app := buffalo.New(buffalo.Options{
		Env:  "development",
		Addr: "127.0.0.1:8000",
	})

	app.GET("/", func(c buffalo.Context) error {
		c.Response().WriteHeader(200)
		_, err := c.Response().Write([]byte("Hello, world!"))
		return err
	})

	log.Fatal(app.Serve())
}
