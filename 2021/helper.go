package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
)

func downloadHelper(year int, day int) string {
	var sessionId = os.Getenv("ADVENT_AUTH_SESSION_ID")
	var cookieHeader = fmt.Sprintf("session=%s", sessionId)
	var url = fmt.Sprintf(
		"https://adventofcode.com/%d/day/%d/input", year, day)

	client := &http.Client{}
	req, _ := http.NewRequest("GET", url, nil)
	req.Header.Set("Cookie", cookieHeader)
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()

	bodyBytes, err := io.ReadAll(resp.Body)

	if err != nil {
		panic(err)
	}
	return string(bodyBytes)
}

func minmax(a, b int) (int, int) {
	if a < b {
		return a, b
	}
	return b, a
}
