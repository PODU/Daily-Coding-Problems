// First recurring character: single pass with a hash set, return first char already seen.
// Time O(n), Space O(k).
package main

import "fmt"

// Returns the recurring rune and true, or false if none.
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

func run(s string) {
	if r, ok := firstRecurring(s); ok {
		fmt.Printf("%c\n", r)
	} else {
		fmt.Println("null")
	}
}

func main() {
	run("acbbac")
	run("abcdef")
}
