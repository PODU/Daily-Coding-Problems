// Day 275: Nth term of look-and-say (term 1 = "1").
// Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func lookAndSay(n int) string {
	cur := "1"
	for t := 1; t < n; t++ {
		var nxt strings.Builder
		i, m := 0, len(cur)
		for i < m {
			j := i
			for j < m && cur[j] == cur[i] {
				j++
			}
			nxt.WriteString(strconv.Itoa(j - i))
			nxt.WriteByte(cur[i])
			i = j
		}
		cur = nxt.String()
	}
	return cur
}

func main() {
	fmt.Println(lookAndSay(5)) // 111221
}
