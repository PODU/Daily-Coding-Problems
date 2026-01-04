// Day 850: De Bruijn sequence via the FKM (Lyndon-word) algorithm.
// Lexicographically smallest cyclic sequence containing every length-n string once. O(k^n).
package main

import (
	"fmt"
	"strings"
)

func deBruijn(k, n int, alphabet string) string {
	a := make([]int, k*n)
	var seq []int
	var db func(t, p int)
	db = func(t, p int) {
		if t > n {
			if n%p == 0 {
				for j := 1; j <= p; j++ {
					seq = append(seq, a[j])
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
	var sb strings.Builder
	for _, i := range seq {
		sb.WriteByte(alphabet[i])
	}
	return sb.String()
}

func main() {
	// C = {0,1}, length 3 => alphabet size 2, n = 3
	fmt.Println(deBruijn(2, 3, "01")) // 00010111
}
