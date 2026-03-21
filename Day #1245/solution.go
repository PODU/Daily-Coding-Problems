// Full text justification: greedy packing + even space distribution (extras
// to the left). Time O(total chars), Space O(output).
package main

import (
	"fmt"
	"strings"
)

func justify(words []string, k int) []string {
	var lines []string
	i, n := 0, len(words)
	for i < n {
		j, length := i, 0
		for j < n && length+len(words[j])+(j-i) <= k {
			length += len(words[j])
			j++
		}
		gaps := j - i - 1
		var line string
		if gaps == 0 {
			line = words[i] + strings.Repeat(" ", k-len(words[i]))
		} else {
			spaces := k - length
			base, extra := spaces/gaps, spaces%gaps
			var sb strings.Builder
			for w := i; w < j-1; w++ {
				sb.WriteString(words[w])
				sp := base
				if (w - i) < extra {
					sp++
				}
				sb.WriteString(strings.Repeat(" ", sp))
			}
			sb.WriteString(words[j-1])
			line = sb.String()
		}
		lines = append(lines, line)
		i = j
	}
	return lines
}

func main() {
	words := []string{"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"}
	for _, line := range justify(words, 16) {
		fmt.Printf("\"%s\"\n", line)
	}
}
