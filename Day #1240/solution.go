// Reflected binary Gray code: g(i) = i ^ (i>>1). Time O(2^n), Space O(2^n).
package main

import (
	"fmt"
	"strings"
)

func grayCode(n int) []string {
	var res []string
	for i := 0; i < (1 << n); i++ {
		g := i ^ (i >> 1)
		bits := make([]byte, n)
		for b := 0; b < n; b++ {
			if (g>>(n-1-b))&1 == 1 {
				bits[b] = '1'
			} else {
				bits[b] = '0'
			}
		}
		res = append(res, string(bits))
	}
	return res
}

func main() {
	fmt.Println("[" + strings.Join(grayCode(2), ", ") + "]")
}
