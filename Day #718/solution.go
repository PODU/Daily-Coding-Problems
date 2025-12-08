// Day 718: Gray code for n bits via formula g(i) = i ^ (i >> 1). Produces 2^n
// values where consecutive (and wrap-around) differ by one bit. Time O(2^n).
package main

import (
	"fmt"
	"strings"
)

func grayCode(n int) []string {
	res := []string{}
	for i := 0; i < (1 << n); i++ {
		g := i ^ (i >> 1)
		var sb strings.Builder
		for b := n - 1; b >= 0; b-- {
			if (g>>b)&1 == 1 {
				sb.WriteByte('1')
			} else {
				sb.WriteByte('0')
			}
		}
		res = append(res, sb.String())
	}
	return res
}

func main() {
	codes := grayCode(2)
	fmt.Println("[" + strings.Join(codes, ", ") + "]")
}
