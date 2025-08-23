// Day 159: First recurring character. Scan left to right tracking seen chars in
// a set; return the first already seen. Time O(n), Space O(alphabet).
package main

import "fmt"

// returns (char, found)
func firstRecurring(s string) (rune, bool) {
	seen := make(map[rune]bool)
	for _, c := range s {
		if seen[c] {
			return c, true
		}
		seen[c] = true
	}
	return 0, false
}

func show(s string) string {
	if c, ok := firstRecurring(s); ok {
		return string(c)
	}
	return "null"
}

func main() {
	fmt.Println(show("acbbac")) // b
	fmt.Println(show("abcdef")) // null
}
