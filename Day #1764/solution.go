// Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
// collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
// Time: O(n) per name, Space: O(n).
package main

import (
	"fmt"
	"unicode"
)

func code(c rune) int {
	c = unicode.ToLower(c)
	switch c {
	case 'b', 'f', 'p', 'v':
		return 1
	case 'c', 'g', 'j', 'k', 'q', 's', 'x', 'z':
		return 2
	case 'd', 't':
		return 3
	case 'l':
		return 4
	case 'm', 'n':
		return 5
	case 'r':
		return 6
	default:
		return 0 // vowels a,e,i,o,u,y and h,w
	}
}

func soundex(name string) string {
	var s []rune
	for _, c := range name {
		if unicode.IsLetter(c) {
			s = append(s, c)
		}
	}
	if len(s) == 0 {
		return ""
	}
	res := []rune{unicode.ToUpper(s[0])}
	prev := code(s[0])
	for i := 1; i < len(s) && len(res) < 4; i++ {
		ch := s[i]
		d := code(ch)
		if d != 0 && d != prev {
			res = append(res, rune('0'+d))
		}
		lc := unicode.ToLower(ch)
		if lc != 'h' && lc != 'w' {
			prev = d
		}
	}
	for len(res) < 4 {
		res = append(res, '0')
	}
	return string(res[:4])
}

func main() {
	fmt.Println(soundex("Jackson"))
	fmt.Println(soundex("Jaxen"))
}
