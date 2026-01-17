// Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.
package main

import (
	"fmt"
	"strings"
)

func grayCode(n int) []string {
	total := 1 << n
	res := make([]string, total)
	for i := 0; i < total; i++ {
		g := i ^ (i >> 1)
		var sb strings.Builder
		for b := n - 1; b >= 0; b-- {
			if (g>>b)&1 == 1 {
				sb.WriteByte('1')
			} else {
				sb.WriteByte('0')
			}
		}
		res[i] = sb.String()
	}
	return res
}

func main() {
	n := 2
	fmt.Println("[" + strings.Join(grayCode(n), ", ") + "]")
}
