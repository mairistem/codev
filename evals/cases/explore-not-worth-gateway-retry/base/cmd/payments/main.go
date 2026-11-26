package main

import (
	"log"
	"net/http"
	"time"

	"example.com/payments/internal/gateway"
)

func main() {
	client := gateway.New("https://gateway.bank.example/v2", &http.Client{Timeout: 3 * time.Second})
	log.Fatal(http.ListenAndServe(":8080", gateway.Handler(client)))
}
