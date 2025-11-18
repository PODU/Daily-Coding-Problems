// Full text justification: greedily pack words per line, distribute spaces evenly
// with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).
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
			j++
			lineLen += 1 + len(words[j])
		}
		gaps := j - i
		var sb strings.Builder
		if gaps == 0 {
			sb.WriteString(words[i])
			sb.WriteString(strings.Repeat(" ", k-len(words[i])))
		} else {
			totalSpaces := k
			for w := i; w <= j; w++ {
				totalSpaces -= len(words[w])
			}
			base := totalSpaces / gaps
			extra := totalSpaces % gaps
			for w := i; w <= j; w++ {
				sb.WriteString(words[w])
				if w < j {
					sp := base
					if w-i < extra {
						sp++
					}
					sb.WriteString(strings.Repeat(" ", sp))
				}
			}
		}
		res = append(res, sb.String())
		i = j + 1
	}
	return res
}

func main() {
	words := []string{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"}
	for _, line := range justify(words, 16) {
		fmt.Printf("\"%s\"\n", line)
	}
}
