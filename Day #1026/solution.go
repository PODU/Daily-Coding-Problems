// Day 1026: Full text justification.
// Greedy line packing; distribute extra spaces evenly, leftover from the left.
// Time O(total chars), Space O(total chars).
package main

import (
	"fmt"
	"strings"
)

func justify(words []string, k int) []string {
	var res []string
	n := len(words)
	i := 0
	for i < n {
		j := i
		lineLen := len(words[i])
		for j+1 < n && lineLen+1+len(words[j+1]) <= k {
			lineLen += 1 + len(words[j+1])
			j++
		}
		cnt := j - i + 1
		if cnt == 1 {
			res = append(res, words[i]+strings.Repeat(" ", k-len(words[i])))
		} else {
			totalChars := 0
			for w := i; w <= j; w++ {
				totalChars += len(words[w])
			}
			spaces := k - totalChars
			gaps := cnt - 1
			base := spaces / gaps
			extra := spaces % gaps
			var sb strings.Builder
			for w := i; w <= j; w++ {
				sb.WriteString(words[w])
				if w < j {
					s := base
					if w-i < extra {
						s++
					}
					sb.WriteString(strings.Repeat(" ", s))
				}
			}
			res = append(res, sb.String())
		}
		i = j + 1
	}
	return res
}

func main() {
	words := []string{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"}
	for _, line := range justify(words, 16) {
		fmt.Println(line)
	}
}
