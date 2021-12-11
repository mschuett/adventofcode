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

// stack implementation from http://tobin.cc/blog/stack/

// Stack of bytes
type Stack []byte

// IsEmpty: check if stack is empty
func (s *Stack) IsEmpty() bool {
	return len(*s) == 0
}

// Push a new byte onto the stack
func (s *Stack) Push(x byte) {
	*s = append(*s, x)
}

// Pop: remove and return top element of stack, return false if stack is empty
func (s *Stack) Pop() (byte, bool) {
	if s.IsEmpty() {
		return 0, false
	}

	i := len(*s) - 1
	x := (*s)[i]
	*s = (*s)[:i]

	return x, true
}

// Peek: return top element of stack, return false if stack is empty
func (s *Stack) Peek() (byte, bool) {
	if s.IsEmpty() {
		return 0, false
	}

	i := len(*s) - 1
	x := (*s)[i]

	return x, true
}
