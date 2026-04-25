// Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
// Approach: greedy single pass over words. O(n) time, O(n) space. nil if a word > k.
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
			return nil, false // impossible
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
	out, ok := wrap("the quick brown fox jumps over the lazy dog", 10)
	if !ok {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(out))
	for i, l := range out {
		parts[i] = "\"" + l + "\""
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
