// Day 604: Soundex phonetic encoding (letter + 3 digits).
// Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).
package main

import (
	"fmt"
	"strings"
)

func code(c byte) byte {
	switch c {
	case 'B', 'F', 'P', 'V':
		return '1'
	case 'C', 'G', 'J', 'K', 'Q', 'S', 'X', 'Z':
		return '2'
	case 'D', 'T':
		return '3'
	case 'L':
		return '4'
	case 'M', 'N':
		return '5'
	case 'R':
		return '6'
	case 'H', 'W':
		return 'S' // transparent
	default:
		return '0' // vowels
	}
}

func soundex(name string) string {
	var up []byte
	for _, r := range strings.ToUpper(name) {
		if r >= 'A' && r <= 'Z' {
			up = append(up, byte(r))
		}
	}
	if len(up) == 0 {
		return "0000"
	}
	res := []byte{up[0]}
	prev := code(up[0])
	for i := 1; i < len(up) && len(res) < 4; i++ {
		c := code(up[i])
		if c >= '1' && c <= '6' {
			if c != prev {
				res = append(res, c)
			}
			prev = c
		} else if c == '0' {
			prev = '0'
		}
	}
	for len(res) < 4 {
		res = append(res, '0')
	}
	return string(res[:4])
}

func main() {
	fmt.Println(soundex("Jackson")) // J250
	fmt.Println(soundex("Jaxen"))   // J250
}
