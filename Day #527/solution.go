// Day 527: Count distinct max-heaps from N distinct integers.
// Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
// binary tree with n nodes. math/big for exactness. Time O(n^2), space O(n).
package main

import (
	"fmt"
	"math/big"
)

// number of nodes in the left subtree of a complete binary tree with n nodes
func leftSubtreeSize(n int) int {
	if n <= 1 {
		return 0
	}
	h := 0
	for (1<<(h+1))-1 <= n {
		h++ // h = height (root at height 0)
	}
	lastLevelCap := 1 << h
	nodesAbove := (1 << h) - 1
	lastLevelNodes := n - nodesAbove
	leftBase := (1 << (h - 1)) - 1
	leftLast := lastLevelNodes
	if lastLevelCap/2 < leftLast {
		leftLast = lastLevelCap / 2
	}
	return leftBase + leftLast
}

func countHeaps(n int) *big.Int {
	if n <= 1 {
		return big.NewInt(1)
	}
	L := leftSubtreeSize(n)
	R := n - 1 - L
	c := new(big.Int).Binomial(int64(n-1), int64(L))
	c.Mul(c, countHeaps(L))
	c.Mul(c, countHeaps(R))
	return c
}

func main() {
	N := 3
	_ = []int{1, 2, 3} // integers
	fmt.Println(countHeaps(N).String()) // expected: 2
}
