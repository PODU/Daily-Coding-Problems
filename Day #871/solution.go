// Soundex phonetic encoding: keep first letter, code rest, dedupe, pad to 3 digits.
// Time O(n), Space O(1) extra per name.
package main

import (
	"fmt"
	"unicode"
)

func code(c rune) int {
	switch unicode.ToLower(c) {
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
		return 0
	}
}

func soundex(name string) string {
	rs := []rune(name)
	i := 0
	for i < len(rs) && !unicode.IsLetter(rs[i]) {
		i++
	}
	if i >= len(rs) {
		return ""
	}
	res := []rune{unicode.ToUpper(rs[i])}
	prev := code(rs[i])
	for j := i + 1; j < len(rs) && len(res) < 4; j++ {
		c := unicode.ToLower(rs[j])
		if !unicode.IsLetter(c) {
			continue
		}
		if c == 'h' || c == 'w' {
			continue
		}
		d := code(c)
		if d == 0 {
			prev = 0
			continue
		}
		if d != prev {
			res = append(res, rune('0'+d))
		}
		prev = d
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
