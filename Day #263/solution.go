// Day 263: Sentence checker over a stream of characters.
// Approach: validate each candidate sentence with a regex enforcing rules 2-4,
// plus a manual check of rule 1 (RE2 has no lookahead). Time O(n) per sentence.

package main

import (
	"fmt"
	"regexp"
)

// Rules:
// 1. Starts with a capital letter followed by a lowercase letter or a space.
// 2. Other chars: lowercase letters, separators (, ; :) or terminal marks (. ? ! ‽).
// 3. A single space between each word.
// 4. Ends with a terminal mark immediately following a word.
var re = regexp.MustCompile(`^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$`)

func isValid(s string) bool {
	r := []rune(s)
	if len(r) == 0 || r[0] < 'A' || r[0] > 'Z' {
		return false
	}
	// Rule 1: char after the capital must be lowercase or space.
	if len(r) >= 2 {
		c := r[1]
		if !((c >= 'a' && c <= 'z') || c == ' ') {
			return false
		}
	}
	return re.MatchString(s)
}

func main() {
	tests := []string{
		"Hello world.",
		"hello world",
		"Hello  world.",
		"The quick, brown fox jumps.",
		"Is this valid?",
		"No terminal mark",
		"Bad ,spacing.",
	}
	for _, t := range tests {
		if isValid(t) {
			fmt.Println(t)
		}
	}
}
