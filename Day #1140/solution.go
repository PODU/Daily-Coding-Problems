// De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
package main

import (
	"fmt"
	"strings"
)

func deBruijn(k, n int) string {
	a := make([]int, k*n+1)
	var sb strings.Builder

	var db func(t, p int)
	db = func(t, p int) {
		if t > n {
			if n%p == 0 {
				for i := 1; i <= p; i++ {
					sb.WriteByte(byte('0' + a[i]))
				}
			}
		} else {
			a[t] = a[t-p]
			db(t+1, p)
			for j := a[t-p] + 1; j < k; j++ {
				a[t] = j
				db(t+1, t)
			}
		}
	}

	db(1, 1)
	return sb.String()
}

func main() {
	// C = {0,1} -> alphabet size 2, substring length 3
	fmt.Println(deBruijn(2, 3))
}
