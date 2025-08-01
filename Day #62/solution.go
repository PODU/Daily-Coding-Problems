// Count lattice paths in N x M grid via combinatorics C(n+m-2, n-1).
// Time O(min(n,m)) multiplicative, Space O(1).
package main

import "fmt"

func paths(n, m int) int64 {
	down, right := n-1, m-1
	k, total := down, down+right
	if right < k {
		k = right
	}
	var res int64 = 1
	for i := 1; i <= k; i++ {
		res = res * int64(total-k+i) / int64(i)
	}
	return res
}

func main() {
	fmt.Printf("2 by 2 matrix -> %d\n", paths(2, 2))
	fmt.Printf("5 by 5 matrix -> %d\n", paths(5, 5))
}
