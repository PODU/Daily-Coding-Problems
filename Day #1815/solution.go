// Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
// Time: O(n * 2^n) to format. Space: O(2^n).
package main

import (
	"fmt"
	"strings"
)

func grayCode(n int) []string {
	res := []string{}
	for i := 0; i < (1 << n); i++ {
		g := i ^ (i >> 1)
		s := fmt.Sprintf("%0*b", n, g)
		res = append(res, s)
	}
	return res
}

func main() {
	fmt.Println("[" + strings.Join(grayCode(2), ", ") + "]") // [00, 01, 11, 10]
}
