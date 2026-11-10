// Command linkcheck reports the broken links of a Markdown file.
package main

import (
	"fmt"
	"net/http"
	"os"
	"time"

	"example.com/linkcheck/internal/links"
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
	client := &http.Client{Timeout: 10 * time.Second}
	broken := links.Check(client, links.Extract(string(text)))
	for _, b := range broken {
		fmt.Println(b)
	}
	if len(broken) > 0 {
		os.Exit(1)
	}
}
