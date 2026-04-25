// Day 1424: URL shortener (shorten -> 6-char code, restore -> original or null).
// Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.
package main

import "fmt"

const alphabet = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

type URLShortener struct {
	counter    int
	urlToShort map[string]string
	shortToUrl map[string]string
}

func NewURLShortener() *URLShortener {
	return &URLShortener{
		urlToShort: map[string]string{},
		shortToUrl: map[string]string{},
	}
}

func (u *URLShortener) encode(n int) string {
	s := []byte("000000")
	for i := 5; i >= 0; i-- {
		s[i] = alphabet[n%62]
		n /= 62
	}
	return string(s)
}

func (u *URLShortener) shorten(url string) string {
	if code, ok := u.urlToShort[url]; ok {
		return code // same URL -> same code
	}
	code := u.encode(u.counter)
	u.counter++
	u.urlToShort[url] = code
	u.shortToUrl[code] = url
	return code
}

func (u *URLShortener) restore(code string) string {
	if url, ok := u.shortToUrl[code]; ok {
		return url
	}
	return "null"
}

func main() {
	s := NewURLShortener()
	a := s.shorten("https://example.com/page")
	b := s.shorten("https://example.com/page")
	fmt.Println(a)        // 000000
	fmt.Println(a == b)   // true
	fmt.Println(s.restore(a))        // https://example.com/page
	fmt.Println(s.restore("zzzzzz")) // null
}
