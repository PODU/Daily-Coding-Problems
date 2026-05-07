// Day 1481: Reverse words while keeping delimiters in their original positions.
// Tokenize into word-runs and delimiter chars, reverse word tokens, re-emit.
// Handles trailing/consecutive delimiters. Time O(N), Space O(N).
package main

import (
	"fmt"
	"strings"
)

type token struct {
	word bool
	text string
}

func reverseWords(s string, delims map[rune]bool) string {
	r := []rune(s)
	var tokens []token
	i, n := 0, len(r)
	for i < n {
		if delims[r[i]] {
			tokens = append(tokens, token{false, string(r[i])})
			i++
		} else {
			j := i
			for j < n && !delims[r[j]] {
				j++
			}
			tokens = append(tokens, token{true, string(r[i:j])})
			i = j
		}
	}
	var words []string
	for _, t := range tokens {
		if t.word {
			words = append(words, t.text)
		}
	}
	// reverse
	for a, b := 0, len(words)-1; a < b; a, b = a+1, b-1 {
		words[a], words[b] = words[b], words[a]
	}
	var out strings.Builder
	k := 0
	for _, t := range tokens {
		if t.word {
			out.WriteString(words[k])
			k++
		} else {
			out.WriteString(t.text)
		}
	}
	return out.String()
}

func main() {
	d := map[rune]bool{'/': true, ':': true}
	fmt.Println(reverseWords("hello/world:here", d))   // here/world:hello
	fmt.Println(reverseWords("hello/world:here/", d))  // here/world:hello/
	fmt.Println(reverseWords("hello//world:here", d))  // here//world:hello
}
