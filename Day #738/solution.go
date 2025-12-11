// Greedy line wrapping: fit max words per line within width k.
// Return nil if any single word exceeds k.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

func wrap(s string, k int) []string {
	words := strings.Fields(s)
	var out []string
	line := ""
	for _, word := range words {
		if len(word) > k {
			return nil
		}
		if line == "" {
			line = word
		} else if len(line)+1+len(word) <= k {
			line += " " + word
		} else {
			out = append(out, line)
			line = word
		}
	}
	if line != "" {
		out = append(out, line)
	}
	return out
}

func main() {
	out := wrap("the quick brown fox jumps over the lazy dog", 10)
	if out == nil {
		fmt.Println("null")
		return
	}
	fmt.Printf("[\"%s\"]\n", strings.Join(out, "\", \""))
	// ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
}
