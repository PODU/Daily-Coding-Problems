// Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).
package main

import (
	"fmt"
	"strings"
)

func grayCode(n int) []string {
	res := make([]string, 0, 1<<n)
	for i := 0; i < (1 << n); i++ {
		g := i ^ (i >> 1)
		s := make([]byte, n)
		for b := 0; b < n; b++ {
			if g&(1<<(n-1-b)) != 0 {
				s[b] = '1'
			} else {
				s[b] = '0'
			}
		}
		res = append(res, string(s))
	}
	return res
}

func main() {
	n := 2
	fmt.Println("[" + strings.Join(grayCode(n), ", ") + "]")
}
