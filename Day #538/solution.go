// De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
// length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).
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
				for j := 1; j <= p; j++ {
					sb.WriteByte(byte('0' + a[j]))
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
	fmt.Println(deBruijn(2, 3)) // 00010111
}
