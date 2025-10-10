// Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
// Time O(5^(N/2)) results, Space O(N) recursion depth.
package main

import (
	"fmt"
	"strings"
)

var pairs = [][2]string{{"0", "0"}, {"1", "1"}, {"6", "9"}, {"8", "8"}, {"9", "6"}}

func build(n, total int) []string {
	if n == 0 {
		return []string{""}
	}
	if n == 1 {
		return []string{"0", "1", "8"}
	}
	inner := build(n-2, total)
	res := []string{}
	for _, s := range inner {
		for _, p := range pairs {
			if n == total && p[0] == "0" { // no leading zero
				continue
			}
			res = append(res, p[0]+s+p[1])
		}
	}
	return res
}

func main() {
	res := build(2, 2)
	quoted := make([]string, len(res))
	for i, x := range res {
		quoted[i] = "\"" + x + "\""
	}
	fmt.Println("[" + strings.Join(quoted, ", ") + "]")
}
