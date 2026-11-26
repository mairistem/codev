package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestOneBrokenLink(t *testing.T) {
	srv := httptest.NewServer(http.NotFoundHandler())
	defer srv.Close()
	got := Check("# Notes\n\nSee "+srv.URL+"/gone.\n", srv.Client())
	if len(got) != 1 || got[0].Line != 3 || got[0].Status != "404" {
		t.Fatalf("got %+v", got)
	}
}
