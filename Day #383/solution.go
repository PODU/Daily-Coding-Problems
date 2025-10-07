// Mark covered indices for every substring occurrence, then wrap maximal runs.
// Time: O(|s| * sum|sub|), Space: O(|s|).
package main

import (
	"fmt"
	"strings"
)

func embolden(s string, lst []string) string {
	n := len(s)
	bold := make([]bool, n)
	for _, sub := range lst {
		if sub == "" {
			continue
		}
		from := 0
		for {
			idx := strings.Index(s[from:], sub)
			if idx == -1 {
				break
			}
			pos := from + idx
			for i := pos; i < pos+len(sub); i++ {
				bold[i] = true
			}
			from = pos + 1
		}
	}
	var out strings.Builder
	for i := 0; i < n; i++ {
		if bold[i] && (i == 0 || !bold[i-1]) {
			out.WriteString("<b>")
		}
		out.WriteByte(s[i])
		if bold[i] && (i == n-1 || !bold[i+1]) {
			out.WriteString("</b>")
		}
	}
	return out.String()
}

func main() {
	fmt.Println(embolden("abcdefg", []string{"bc", "ef"}))
	fmt.Println(embolden("abcdefg", []string{"bcd", "def"}))
}
