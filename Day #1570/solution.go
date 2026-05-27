// Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func lookAndSay(n int) string {
	cur := "1"
	for k := 1; k < n; k++ {
		var next strings.Builder
		i, length := 0, len(cur)
		for i < length {
			j := i
			for j < length && cur[j] == cur[i] {
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
	fmt.Println(lookAndSay(4)) // 1, 11, 21, 1211
}
