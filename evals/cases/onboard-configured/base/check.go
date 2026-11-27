package main

import (
	"net/http"
	"regexp"
	"strconv"
	"strings"
)

var link = regexp.MustCompile(`https?://[^\s)>\]]+`)

// Broken is a link that answered 400 or more, or did not answer.
type Broken struct {
	Line   int
	URL    string
	Status string
}

// Check returns the broken links of text, in order.
func Check(text string, client *http.Client) []Broken {
	var out []Broken
	for i, line := range strings.Split(text, "\n") {
		for _, url := range link.FindAllString(line, -1) {
			resp, err := client.Head(url)
			if err != nil {
				out = append(out, Broken{i + 1, url, "unreachable"})
				continue
			}
			resp.Body.Close()
			if resp.StatusCode >= 400 {
				out = append(out, Broken{i + 1, url, strconv.Itoa(resp.StatusCode)})
			}
		}
	}
	return out
}
