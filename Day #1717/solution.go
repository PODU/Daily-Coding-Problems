// Day 1717: Fully justify text into lines of length k.
// Greedy line packing + even space distribution (extras from left).
// Time: O(total characters), Space: O(output).
package main

import (
	"fmt"
	"strings"
)

func justify(words []string, k int) []string {
	var lines []string
	n := len(words)
	i := 0
	for i < n {
		j := i
		lineLen := len(words[i])
		for j+1 < n && lineLen+1+len(words[j+1]) <= k {
			j++
			lineLen += 1 + len(words[j])
		}
		cnt := j - i + 1
		wordChars := 0
		for t := i; t <= j; t++ {
			wordChars += len(words[t])
		}
		var sb strings.Builder
		if cnt == 1 {
			sb.WriteString(words[i])
			sb.WriteString(strings.Repeat(" ", k-len(words[i])))
		} else {
			gaps := cnt - 1
			totalSpaces := k - wordChars
			base := totalSpaces / gaps
			extra := totalSpaces % gaps
			for t := i; t <= j; t++ {
				sb.WriteString(words[t])
				if t < j {
					sp := base
					if t-i < extra {
						sp++
					}
					sb.WriteString(strings.Repeat(" ", sp))
				}
			}
		}
		lines = append(lines, sb.String())
		i = j + 1
	}
	return lines
}

func main() {
	words := []string{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"}
	for _, l := range justify(words, 16) {
		fmt.Printf("\"%s\"\n", l)
	}
}
