// Sentence validator over a char stream: split on terminal marks, validate each candidate.
// isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).
// Demo uses ASCII terminals . ? ! (interrobang treated as terminal if present).
package main

import (
	"fmt"
	"strings"
)

func isTerminal(c byte) bool { return c == '.' || c == '?' || c == '!' }
func isSeparator(c byte) bool { return c == ',' || c == ';' || c == ':' }
func isLower(c byte) bool      { return c >= 'a' && c <= 'z' }
func isUpper(c byte) bool      { return c >= 'A' && c <= 'Z' }

func isValid(s string) bool {
	n := len(s)
	if n < 2 {
		return false
	}
	if !isUpper(s[0]) {
		return false
	}
	if !(isLower(s[1]) || s[1] == ' ') {
		return false
	}
	if !isTerminal(s[n-1]) {
		return false
	}
	if !(isLower(s[n-2]) || isUpper(s[n-2])) {
		return false
	}
	for i := 1; i+1 < n; i++ {
		c := s[i]
		if isLower(c) || isSeparator(c) {
			continue
		}
		if c == ' ' {
			if s[i-1] == ' ' {
				return false
			}
			continue
		}
		return false
	}
	return true
}

func main() {
	stream := "Hello world. this is bad."
	var buf strings.Builder
	for i := 0; i < len(stream); i++ {
		ch := stream[i]
		buf.WriteByte(ch)
		if isTerminal(ch) {
			candidate := strings.TrimSpace(buf.String())
			if isValid(candidate) {
				fmt.Println(candidate)
			}
			buf.Reset()
		}
	}
}
