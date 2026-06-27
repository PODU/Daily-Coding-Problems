// Reverse words while keeping delimiters in place: split into word/delimiter tokens,
// reverse only the word list, re-emit in original token order. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type token struct {
	text   string
	isWord bool
}

func reverseWords(s string, delims map[rune]bool) string {
	var tokens []token
	var cur strings.Builder
	for _, c := range s {
		if delims[c] {
			if cur.Len() > 0 {
				tokens = append(tokens, token{cur.String(), true})
				cur.Reset()
			}
			tokens = append(tokens, token{string(c), false})
		} else {
			cur.WriteRune(c)
		}
	}
	if cur.Len() > 0 {
		tokens = append(tokens, token{cur.String(), true})
	}

	var words []string
	for _, t := range tokens {
		if t.isWord {
			words = append(words, t.text)
		}
	}
	for i, j := 0, len(words)-1; i < j; i, j = i+1, j-1 {
		words[i], words[j] = words[j], words[i]
	}

	var res strings.Builder
	wi := 0
	for _, t := range tokens {
		if t.isWord {
			res.WriteString(words[wi])
			wi++
		} else {
			res.WriteString(t.text)
		}
	}
	return res.String()
}

func main() {
	delims := map[rune]bool{'/': true, ':': true}
	fmt.Println(reverseWords("hello/world:here", delims))
	fmt.Println(reverseWords("hello/world:here/", delims))
	fmt.Println(reverseWords("hello//world:here", delims))
}
