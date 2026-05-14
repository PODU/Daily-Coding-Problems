// First recurring character: scan L->R, track seen set; first char already seen wins.
// O(n) time, O(alphabet) space.
package main

import "fmt"

// Returns the first recurring rune and true, or 0 and false if none.
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
	r, found := firstRecurring("acbbac")
	if found {
		fmt.Printf("%c\n", r)
	} else {
		fmt.Println("null")
	}
}
