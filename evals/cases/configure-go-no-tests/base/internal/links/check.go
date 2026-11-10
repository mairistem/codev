package links

import (
	"fmt"
	"net/http"
)

// Check returns one line per link that fails: a transport error or a
// status of 400 and above.
func Check(client *http.Client, urls []string) []string {
	var broken []string
	for _, url := range urls {
		resp, err := client.Head(url)
		if err != nil {
			broken = append(broken, fmt.Errorf("%s: %w", url, err).Error())
			continue
		}
		resp.Body.Close()
		if resp.StatusCode >= 400 {
			broken = append(broken, fmt.Sprintf("%s: %d", url, resp.StatusCode))
		}
	}
	return broken
}
