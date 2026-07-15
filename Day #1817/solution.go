// De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
// Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).
package main

import (
	"fmt"
	"strings"
)

func deBruijn(C []string, n int) string {
	k := len(C)
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
		sb.WriteString(C[i])
	}
	return sb.String()
}

func main() {
	fmt.Println(deBruijn([]string{"0", "1"}, 3)) // 00010111
}
