package main

import (
	"log"
	"strconv"
	"time"

	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/adaptor"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	httpServerRequestSeconds = promauto.NewSummaryVec(
		prometheus.SummaryOpts{
			Name: "http_server_requests_seconds",
			Help: "Http Server request seconds",
		},
		[]string{"error", "exception", "method", "outcome", "status", "uri"},
	)
)

func main() {
	// Initialize a new Fiber app
	app := fiber.New()

	router := app.Use(func(c fiber.Ctx) error {
		startTime := time.Now().UnixMilli()
		err := c.Next()
		endTime := time.Now().UnixMilli()

		var diffTime float64 = (float64(endTime) - float64(startTime)) / 1000

		status := c.Response().StatusCode()
		httpServerRequestSeconds.WithLabelValues(
			"none",
			"none",
			string(c.Request().Header.Method()),
			"SUCCESS",
			strconv.Itoa(status),
			string(c.Request().URI().Path()),
		).
			Observe(diffTime)
		return err
	})

	router.Get("/metrics", adaptor.HTTPHandler(promhttp.Handler()))

	// Define a route for the GET method on the root path '/'
	router.Get("/", func(c fiber.Ctx) error {

		// Send a string response to the client
		data := map[string]any{}

		name := c.Query("name", "user")

		data["message"] = "ok"
		data["data"] = "hello " + name
		return c.Status(200).JSON(data)
	})

	// Define a route for the GET method on the root path '/'
	router.Get("/fibonacci", func(c fiber.Ctx) error {
		// Send a string response to the client
		data := map[string]any{}

		nString := c.Query("n", "")
		if nString == "" {
			data["message"] = "n query required"
			data["data"] = nil
			return c.Status(400).JSON(data)
		}

		n, err := strconv.Atoi(nString)

		if err != nil {
			data["message"] = "n must be valid number"
			data["data"] = nil
			return c.Status(400).JSON(data)
		}

		if n < 1 {
			data["message"] = "n must be start from 1"
			data["data"] = nil
			return c.Status(400).JSON(data)
		}

		data["message"] = "ok"
		data["data"] = fibonacci(n)

		return c.Status(200).JSON(data)
	})

	// Start the server on port 8081
	log.Fatal(app.Listen(":8081"))
}

func fibonacci(n int) int64 {
	if n <= 1 {
		return int64(n)
	}

	return fibonacci(n-1) + fibonacci(n-2)
}
