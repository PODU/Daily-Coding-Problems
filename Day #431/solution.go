// Day 431: Sentence validator via finite-state-machine scan over runes.
// Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!‽]$ (no backtracking needed).
// O(n) time, O(n) space per sentence.
package main

import "fmt"

func isSep(c rune) bool  { return c == ',' || c == ';' || c == ':' }
func isTerm(c rune) bool { return c == '.' || c == '?' || c == '!' || c == '‽' }
func isLow(c rune) bool  { return c >= 'a' && c <= 'z' }

func isValidSentence(s string) bool {
	a := []rune(s)
	n := len(a)
	if n == 0 {
		return false
	}
	if !(a[0] >= 'A' && a[0] <= 'Z') {
		return false
	}
	i := 1
	for i < n && isLow(a[i]) {
		i++
	}
	for {
		j := i
		if j < n && isSep(a[j]) {
			j++
		}
		if j < n && a[j] == ' ' {
			j++
			if j < n && isLow(a[j]) {
				for j < n && isLow(a[j]) {
					j++
				}
				i = j
				continue
			}
		}
		break
	}
	if i < n && isSep(a[i]) {
		i++
	}
	return i == n-1 && isTerm(a[i])
}

func main() {
	tests := []string{"The quick brown fox.", "hello world.", "Hello  world.",
		"Hello world", "Hi there, friend!"}
	for _, t := range tests {
		if isValidSentence(t) {
			fmt.Println("Valid: " + t)
		} else {
			fmt.Println("Invalid: " + t)
		}
	}
}
