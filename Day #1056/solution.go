// Look-and-say sequence: start "1"; each term describes digit runs of previous.
// Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func lookAndSay(n int) string {
	cur := "1"
	for i := 1; i < n; i++ {
		var next strings.Builder
		j := 0
		for j < len(cur) {
			count := 1
			for j+count < len(cur) && cur[j+count] == cur[j] {
				count++
			}
			next.WriteString(strconv.Itoa(count))
			next.WriteByte(cur[j])
			j += count
		}
		cur = next.String()
	}
	return cur
}

func main() {
	N := 5 // terms: 1, 11, 21, 1211, 111221
	fmt.Println(lookAndSay(N))
}
