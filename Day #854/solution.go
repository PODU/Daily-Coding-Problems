// Day 854: greedy word wrap - pack max words per line of length <= k; nil if any word > k.
// Single pass over words. O(total characters).
package main

import (
	"fmt"
	"strings"
)

func wrap(s string, k int) ([]string, bool) {
	var out []string
	line := ""
	for _, word := range strings.Split(s, " ") {
		if len(word) > k {
			return nil, false
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
	return out, true
}

func main() {
	r, ok := wrap("the quick brown fox jumps over the lazy dog", 10)
	if !ok {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(r))
	for i, l := range r {
		parts[i] = "\"" + l + "\""
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
	// ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
}
