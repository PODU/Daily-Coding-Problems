// Day 1456: First recurring character in a string.
// Approach: scan left-to-right, track seen chars in a set; first char already
// seen is the answer. Time O(n), Space O(1) (fixed alphabet).
package main

import "fmt"

// firstRecurring returns the first recurring rune and true, or false if none.
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
}
