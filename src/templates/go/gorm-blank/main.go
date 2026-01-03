package main

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "Hello, world!")
	})

	fmt.Println("Server running at: http://127.0.0.1:8000")
	fmt.Println("Available endpoints:")
	fmt.Println("   GET / - show hello world")

	r.Run("127.0.0.1:8000")
}
