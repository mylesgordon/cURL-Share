package main

import (
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHelloWorld(t *testing.T) {
	w := httptest.NewRecorder()
	req := httptest.NewRequest(http.MethodGet, "/", nil)

	helloWorld(w, req)

	result := w.Result()
	defer result.Body.Close()
	data, err := io.ReadAll(result.Body)
	if err != nil {
		t.Errorf("Error while reading request body: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected hello world got %v", string(data))
	}
}
