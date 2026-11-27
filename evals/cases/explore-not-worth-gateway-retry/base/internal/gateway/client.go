// Package gateway calls the acquiring bank's payment gateway.
package gateway

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
)

// Client charges cards through the gateway.
type Client struct {
	base string
	http *http.Client
}

// New returns a client of the gateway at base.
func New(base string, h *http.Client) *Client {
	return &Client{base: base, http: h}
}

// Charge authorizes amount cents on the card token, retrying a call that
// times out (see retry.go).
func (c *Client) Charge(ctx context.Context, token string, amount int64) error {
	body, _ := json.Marshal(map[string]any{"token": token, "amount": amount})
	return withRetry(ctx, func() error {
		req, err := http.NewRequestWithContext(ctx, http.MethodPost, c.base+"/charges", bytes.NewReader(body))
		if err != nil {
			return err
		}
		resp, err := c.http.Do(req)
		if err != nil {
			return err
		}
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusCreated {
			return fmt.Errorf("gateway: charge refused (%d)", resp.StatusCode)
		}
		return nil
	})
}

// Handler serves POST /charge for the shop's checkout.
func Handler(c *Client) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var in struct {
			Token  string `json:"token"`
			Amount int64  `json:"amount"`
		}
		if err := json.NewDecoder(r.Body).Decode(&in); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		if err := c.Charge(r.Context(), in.Token, in.Amount); err != nil {
			http.Error(w, err.Error(), http.StatusBadGateway)
			return
		}
		w.WriteHeader(http.StatusCreated)
	})
}
