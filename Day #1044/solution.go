// Reverse words but keep delimiters fixed in place: extract words, reverse the list,
// rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
)

var delims = map[rune]bool{'/': true, ':': true}

func reverseWords(s string) string {
	runes := []rune(s)
	var words []string
	var cur strings.Builder
	for _, c := range runes {
		if delims[c] {
			if cur.Len() > 0 {
				words = append(words, cur.String())
				cur.Reset()
			}
		} else {
			cur.WriteRune(c)
		}
	}
	if cur.Len() > 0 {
		words = append(words, cur.String())
	}
	// reverse word list
	for l, r := 0, len(words)-1; l < r; l, r = l+1, r-1 {
		words[l], words[r] = words[r], words[l]
	}

	var out strings.Builder
	wi, i, n := 0, 0, len(runes)
	for i < n {
		if delims[runes[i]] {
			out.WriteRune(runes[i])
			i++
		} else {
			out.WriteString(words[wi])
			wi++
			for i < n && !delims[runes[i]] {
				i++
			}
		}
	}
	return out.String()
}

func main() {
	tests := []string{"hello/world:here", "hello/world:here/", "hello//world:here"}
	for _, t := range tests {
		fmt.Printf("%s -> %s\n", t, reverseWords(t))
	}
}
