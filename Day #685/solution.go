// Reverse words between delimiters while keeping delimiters fixed in position.
// Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.
package main

import (
	"fmt"
	"strings"
)

type token struct {
	isDelim bool
	val     string
}

func reverseWords(s string, delims map[rune]bool) string {
	var tokens []token
	var buf strings.Builder
	for _, c := range s {
		if delims[c] {
			tokens = append(tokens, token{false, buf.String()})
			tokens = append(tokens, token{true, string(c)})
			buf.Reset()
		} else {
			buf.WriteRune(c)
		}
	}
	if buf.Len() > 0 {
		tokens = append(tokens, token{false, buf.String()})
	}

	var words []string
	for _, t := range tokens {
		if !t.isDelim {
			words = append(words, t.val)
		}
	}
	for i, j := 0, len(words)-1; i < j; i, j = i+1, j-1 {
		words[i], words[j] = words[j], words[i]
	}

	var out strings.Builder
	wi := 0
	for _, t := range tokens {
		if t.isDelim {
			out.WriteString(t.val)
		} else {
			out.WriteString(words[wi])
			wi++
		}
	}
	return out.String()
}

func main() {
	d := map[rune]bool{'/': true, ':': true}
	for _, t := range []string{"hello/world:here", "hello/world:here/", "hello//world:here"} {
		fmt.Printf("%s -> %s\n", t, reverseWords(t, d))
	}
}
