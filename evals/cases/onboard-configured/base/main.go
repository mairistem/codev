package main

import (
	"fmt"
	"net/http"
	"os"
	"time"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: linkcheck <file.md>")
		os.Exit(2)
	}
	text, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(2)
	}
	broken := Check(string(text), &http.Client{Timeout: 10 * time.Second})
	for _, b := range broken {
		fmt.Printf("%s:%d %s %s\n", os.Args[1], b.Line, b.Status, b.URL)
	}
	if len(broken) > 0 {
		os.Exit(1)
	}
}
