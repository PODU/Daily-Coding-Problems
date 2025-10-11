// Day 412: Nth term of the look-and-say sequence via run-length encoding.
// Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func lookAndSay(n int) string {
	cur := "1"
	for t := 1; t < n; t++ {
		var next strings.Builder
		i, m := 0, len(cur)
		for i < m {
			j := i
			for j < m && cur[j] == cur[i] {
				j++
			}
			next.WriteString(strconv.Itoa(j - i))
			next.WriteByte(cur[i])
			i = j
		}
		cur = next.String()
	}
	return cur
}

func main() {
	n := 4
	fmt.Println(lookAndSay(n))
}
