// Word wrap greedily: pack max words per line <= k, return nil (null) if any word > k. O(total length) time.
package main

import (
	"fmt"
	"strings"
)

func wordWrap(s string, k int) ([]string, bool) {
	var lines []string
	cur := ""
	for _, w := range strings.Split(s, " ") {
		if len(w) > k {
			return nil, false
		}
		if cur == "" {
			cur = w
		} else if len(cur)+1+len(w) <= k {
			cur += " " + w
		} else {
			lines = append(lines, cur)
			cur = w
		}
	}
	if cur != "" {
		lines = append(lines, cur)
	}
	return lines, true
}

func main() {
	res, ok := wordWrap("the quick brown fox jumps over the lazy dog", 10)
	if !ok {
		fmt.Println("null")
		return
	}
	parts := make([]string, len(res))
	for i, ln := range res {
		parts[i] = "\"" + ln + "\""
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
