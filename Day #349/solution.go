// Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
// (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
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
		return 0 // vowels, y, w, h
	}
}

func soundex(name string) string {
	r := []rune(name)
	var res strings.Builder
	res.WriteRune(unicode.ToUpper(r[0]))
	prev := code(r[0])
	for i := 1; i < len(r) && res.Len() < 4; i++ {
		c := unicode.ToLower(r[i])
		d := code(c)
		if d != 0 && d != prev {
			res.WriteString(fmt.Sprintf("%d", d))
		}
		if c == 'h' || c == 'w' {
			continue // transparent: keep prev
		}
		prev = d // vowels reset prev to 0
	}
	out := res.String()
	for len(out) < 4 {
		out += "0"
	}
	return out[:4]
}

func main() {
	fmt.Println(soundex("Jackson"))
}
