// Soundex phonetic encoding: keep first letter, map consonants to digits,
// collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.
package main

import (
	"fmt"
	"strings"
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
	if name == "" {
		return ""
	}
	r := []rune(name)
	var out strings.Builder
	out.WriteRune(unicode.ToUpper(r[0]))
	prev := code(r[0])
	for i := 1; i < len(r) && out.Len() < 4; i++ {
		lc := unicode.ToLower(r[i])
		c := code(lc)
		if c != 0 && c != prev {
			out.WriteByte(byte('0' + c))
		}
		if lc == 'h' || lc == 'w' {
			continue
		}
		prev = c
	}
	s := out.String()
	for len(s) < 4 {
		s += "0"
	}
	return s[:4]
}

func main() {
	fmt.Println(soundex("Jackson")) // J250
	fmt.Println(soundex("Jaxen"))   // J250
}
