// Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
// Time: O(1) amortized per op, Space: O(n).
package main

import (
	"fmt"
	"math/rand"
	"time"
)

const alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

type URLShortener struct {
	toLong  map[string]string
	toShort map[string]string
	rng     *rand.Rand
}

func NewURLShortener() *URLShortener {
	return &URLShortener{
		toLong:  make(map[string]string),
		toShort: make(map[string]string),
		rng:     rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

func (s *URLShortener) randCode() string {
	b := make([]byte, 6)
	for i := range b {
		b[i] = alpha[s.rng.Intn(len(alpha))]
	}
	return string(b)
}

func (s *URLShortener) shorten(url string) string {
	if code, ok := s.toShort[url]; ok {
		return code // same URL -> same code
	}
	var code string
	for {
		code = s.randCode()
		if _, exists := s.toLong[code]; !exists {
			break
		}
	}
	s.toLong[code] = url
	s.toShort[url] = code
	return code
}

// restore returns the URL and ok=false (null) when unknown.
func (s *URLShortener) restore(code string) (string, bool) {
	url, ok := s.toLong[code]
	return url, ok
}

func main() {
	s := NewURLShortener()
	a := s.shorten("https://example.com/foo")
	b := s.shorten("https://example.com/foo") // same URL twice
	fmt.Println("same code reused:", a == b)
	url, _ := s.restore(a)
	fmt.Println("restore:", url)
	if _, ok := s.restore("zzzzzz"); !ok {
		fmt.Println("restore unknown: null")
	}
}
