// Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
// L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).
package main

import (
	"fmt"
	"math"
)

func leftSubtreeSize(n int) int {
	h := int(math.Floor(math.Log2(float64(n))))
	m := n - ((1 << h) - 1)
	lastCap := 1 << (h - 1)
	if m > lastCap {
		m = lastCap
	}
	return ((1 << (h - 1)) - 1) + m
}

func main() {
	N := 3
	C := make([][]int64, N+1)
	for i := range C {
		C[i] = make([]int64, N+1)
		C[i][0] = 1
		for j := 1; j <= i; j++ {
			C[i][j] = C[i-1][j-1] + C[i-1][j]
		}
	}
	ways := make([]int64, N+1)
	ways[0] = 1
	if N >= 1 {
		ways[1] = 1
	}
	for n := 2; n <= N; n++ {
		L := leftSubtreeSize(n)
		R := n - 1 - L
		ways[n] = C[n-1][L] * ways[L] * ways[R]
	}
	fmt.Println(ways[N])
}
