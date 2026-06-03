// Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
// L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(n^2).
package main

import (
	"fmt"
	"math"
)

var C [64][64]int64

func leftCount(n int) int {
	if n == 1 {
		return 0
	}
	h := int(math.Floor(math.Log2(float64(n)))) // height (root at level 0)
	last := n - ((1 << h) - 1)                   // nodes in the bottom level
	maxLast := 1 << (h - 1)                       // max bottom-level nodes for left subtree
	if last > maxLast {
		last = maxLast
	}
	return ((1 << (h - 1)) - 1) + last
}

func ways(n int) int64 {
	if n <= 1 {
		return 1
	}
	L := leftCount(n)
	R := n - 1 - L
	return C[n-1][L] * ways(L) * ways(R)
}

func main() {
	for i := 0; i < 64; i++ {
		C[i][0] = 1
		for j := 1; j <= i; j++ {
			C[i][j] = C[i-1][j-1] + C[i-1][j]
		}
	}
	arr := []int{1, 2, 3} // N distinct integers
	fmt.Println(ways(len(arr)))
}
