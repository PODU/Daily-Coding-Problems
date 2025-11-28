// Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
// dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
package main

import "fmt"

const alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"

type Shortener struct {
	code2url map[string]string
	url2code map[string]string
	counter  int64
}

func NewShortener() *Shortener {
	return &Shortener{map[string]string{}, map[string]string{}, 916132831}
}
func (s *Shortener) encode(n int64) string {
	b := make([]byte, 6)
	for i := 5; i >= 0; i-- {
		b[i] = alpha[n%62]
		n /= 62
	}
	return string(b)
}
func (s *Shortener) shorten(url string) string {
	if c, ok := s.url2code[url]; ok {
		return c
	}
	code := s.encode(s.counter)
	s.counter++
	s.code2url[code] = url
	s.url2code[url] = code
	return code
}
func (s *Shortener) restore(code string) interface{} {
	if u, ok := s.code2url[code]; ok {
		return u
	}
	return nil
}

func main() {
	s := NewShortener()
	c := s.shorten("https://example.com/long/path")
	fmt.Println("short:", c)
	fmt.Println("restore:", s.restore(c))
	fmt.Println("restore(zzzzzz):", s.restore("zzzzzz")) // <nil>
}
