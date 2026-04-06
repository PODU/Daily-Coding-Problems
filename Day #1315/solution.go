// Look-and-say: build each term by run-length encoding the previous one.
// Time O(sum of term lengths), Space O(length of Nth term).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func lookAndSay(n int) string {
	s := "1"
	for i := 1; i < n; i++ {
		var next strings.Builder
		for j := 0; j < len(s); {
			k := j
			for k < len(s) && s[k] == s[j] {
				k++
			}
			next.WriteString(strconv.Itoa(k - j))
			next.WriteByte(s[j])
			j = k
		}
		s = next.String()
	}
	return s
}

func main() {
	fmt.Println(lookAndSay(5)) // 111221
}
