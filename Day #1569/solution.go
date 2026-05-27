// URL shortener: base62 6-char codes from a counter, two maps (forward + reverse) for dedupe.
// Time O(1) per shorten/restore, space O(n).
package main

import (
	"fmt"
	"strings"
)

const alphabet = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

type URLShortener struct {
	counter   int64
	codeToURL map[string]string
	urlToCode map[string]string
}

func NewURLShortener() *URLShortener {
	u := &URLShortener{
		codeToURL: make(map[string]string),
		urlToCode: make(map[string]string),
	}
	u.counter = u.decode("abcdef")
	return u
}

func (u *URLShortener) encode(num int64) string {
	b := []byte("000000")
	for i := 5; i >= 0; i-- {
		b[i] = alphabet[num%62]
		num /= 62
	}
	return string(b)
}

func (u *URLShortener) decode(s string) int64 {
	var num int64
	for i := 0; i < len(s); i++ {
		num = num*62 + int64(strings.IndexByte(alphabet, s[i]))
	}
	return num
}

func (u *URLShortener) shorten(url string) string {
	if code, ok := u.urlToCode[url]; ok {
		return code
	}
	code := u.encode(u.counter)
	u.counter++
	u.codeToURL[code] = url
	u.urlToCode[url] = code
	return code
}

func (u *URLShortener) restore(code string) (string, bool) {
	url, ok := u.codeToURL[code]
	return url, ok
}

func main() {
	s := NewURLShortener()
	code := s.shorten("https://www.example.com/some/long/path")
	fmt.Println(code)
	if url, ok := s.restore(code); ok {
		fmt.Println(url)
	} else {
		fmt.Println("null")
	}
	if url, ok := s.restore("XXXXXX"); ok {
		fmt.Println(url)
	} else {
		fmt.Println("null")
	}
}
