// URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
// Dedup via url->code map so identical URLs map to the same code.
// shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.
package main

import "fmt"

const alpha = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

type URLShortener struct {
	counter   int64
	urlToCode map[string]string
	codeToURL map[string]string
}

func NewURLShortener() *URLShortener {
	return &URLShortener{
		urlToCode: make(map[string]string),
		codeToURL: make(map[string]string),
	}
}

func encode(n int64) string {
	s := []byte{alpha[0], alpha[0], alpha[0], alpha[0], alpha[0], alpha[0]}
	i := 5
	for n > 0 && i >= 0 {
		s[i] = alpha[n%62]
		n /= 62
		i--
	}
	return string(s)
}

func (u *URLShortener) Shorten(url string) string {
	if code, ok := u.urlToCode[url]; ok { // same URL -> same code
		return code
	}
	code := encode(u.counter)
	u.counter++
	u.urlToCode[url] = code
	u.codeToURL[code] = url
	return code
}

// Restore returns (url, true) or ("", false) if the code is unknown.
func (u *URLShortener) Restore(short string) (string, bool) {
	url, ok := u.codeToURL[short]
	return url, ok
}

func main() {
	s := NewURLShortener()
	url := "https://www.example.com/some/long/path"
	code := s.Shorten(url)
	fmt.Printf("shorten(%s) = %s\n", url, code)

	if v, ok := s.Restore(code); ok {
		fmt.Printf("restore(%s) = %s\n", code, v)
	}
	if _, ok := s.Restore("zzzzzz"); !ok {
		fmt.Println("restore(zzzzzz) = null")
	}
	code2 := s.Shorten(url)
	fmt.Printf("shorten same url again = %s (same as before: %t)\n", code2, code == code2)
}
