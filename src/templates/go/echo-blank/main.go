package main

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()

	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, world!")
	})

	fmt.Println("Server running at: http://127.0.0.1:8000")
	fmt.Println("Available endpoints:")
	fmt.Println("   GET / - show hello world")

	e.Start("127.0.0.1:8000")
}
