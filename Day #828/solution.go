// Day 828: Count distinct max heaps from N distinct integers.
// Root is the max; f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size from
// complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
package main

import (
	"fmt"
	"math/bits"
)

var C [][]int64

func leftCount(m int) int {
	if m == 1 {
		return 0
	}
	h := bits.Len(uint(m)) - 1 // height (0-indexed levels)
	last := m - ((1 << h) - 1)
	leftCap := 1 << (h - 1)
	if last < leftCap {
		return ((1 << (h - 1)) - 1) + last
	}
	return ((1 << (h - 1)) - 1) + leftCap
}

func f(m int) int64 {
	if m <= 1 {
		return 1
	}
	L := leftCount(m)
	R := m - 1 - L
	return C[m-1][L] * f(L) * f(R)
}

func countMaxHeaps(n int) int64 {
	C = make([][]int64, n+1)
	for i := 0; i <= n; i++ {
		C[i] = make([]int64, n+1)
		C[i][0] = 1
		for j := 1; j <= i; j++ {
			C[i][j] = C[i-1][j-1] + C[i-1][j]
		}
	}
	return f(n)
}

func main() {
	fmt.Println(countMaxHeaps(3))
}
