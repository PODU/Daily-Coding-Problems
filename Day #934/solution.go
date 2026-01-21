// Day 934: First recurring character = first char that has been seen before while scanning.
// Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).
package main

import "fmt"

// Returns (char, true) on first repeat, or (0, false) if none.
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

func main() {
	for _, s := range []string{"acbbac", "abcdef"} {
		if c, ok := firstRecurring(s); ok {
			fmt.Printf("\"%c\"\n", c)
		} else {
			fmt.Println("null")
		}
	}
	// "b"
	// null
}
