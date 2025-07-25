// Text justification: greedy line packing, distribute spaces with extras to LEFT gaps.
// Time: O(total chars), Space: O(total chars) for output.
package main

import (
	"fmt"
	"strings"
)

func justify(words []string, k int) []string {
	res := []string{}
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
		var line string
		if gaps == 0 {
			line = words[i] + strings.Repeat(" ", k-len(words[i]))
		} else {
			totalChars := 0
			for w := i; w <= j; w++ {
				totalChars += len(words[w])
			}
			totalSpaces := k - totalChars
			base := totalSpaces / gaps
			extra := totalSpaces % gaps
			var sb strings.Builder
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
			line = sb.String()
		}
		res = append(res, line)
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
