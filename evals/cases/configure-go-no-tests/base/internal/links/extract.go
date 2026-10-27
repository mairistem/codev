// Package links finds the links of a Markdown text and checks them.
package links

import "regexp"

var inline = regexp.MustCompile(`\]\((https?://[^)\s]+)\)`)

// Extract returns the absolute links of text, in order, each once.
func Extract(text string) []string {
	seen := map[string]bool{}
	var out []string
	for _, m := range inline.FindAllStringSubmatch(text, -1) {
		if !seen[m[1]] {
			seen[m[1]] = true
			out = append(out, m[1])
		}
	}
	return out
}
