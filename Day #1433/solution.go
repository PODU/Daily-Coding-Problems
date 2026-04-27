// Day 1433: Sentence checker over a character stream.
// Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.
package main

import (
	"fmt"
	"unicode"
)

func isTerminal(c rune) bool {
	return c == '.' || c == '?' || c == '!' || c == '‽' // ‽
}

func isSeparator(c rune) bool {
	return c == ',' || c == ';' || c == ':'
}

func isValidSentence(s string) bool {
	runes := []rune(s)
	n := len(runes)
	if n < 2 {
		return false
	}
	// Rule 1: capital then lowercase or space.
	if !unicode.IsUpper(runes[0]) {
		return false
	}
	if !(unicode.IsLower(runes[1]) || runes[1] == ' ') {
		return false
	}

	prevWasLetter := unicode.IsLetter(runes[0])
	for i := 1; i < n; i++ {
		c := runes[i]
		if isTerminal(c) {
			if !prevWasLetter { // Rule 4
				return false
			}
			return i == n-1
		}
		switch {
		case c == ' ':
			if runes[i-1] == ' ' { // Rule 3: single space
				return false
			}
			prevWasLetter = false
		case unicode.IsLower(c):
			prevWasLetter = true
		case isSeparator(c):
			prevWasLetter = false
		default:
			return false
		}
	}
	return false // no terminal mark
}

func main() {
	tests := []string{
		"The quick brown fox.",
		"Hello world!",
		"lowercase start.",
		"No terminal mark",
		"Two  spaces here.",
	}
	for _, t := range tests {
		if isValidSentence(t) {
			fmt.Println(t)
		}
	}
}
