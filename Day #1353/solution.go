// Count max-heaps: count(n)=C(n-1,L)*count(L)*count(R), L=left subtree size from heap shape.
// Time: O(N^2) (binomial table + recursion), Space: O(N^2).
package main

import (
	"fmt"
	"math"
)

var C [][]int64

func leftSize(n int) int {
	if n == 1 {
		return 0
	}
	h := int(math.Floor(math.Log2(float64(n))))
	lower := 1 << (h - 1)
	last := n - ((1 << h) - 1)
	leftLast := last
	if lower < leftLast {
		leftLast = lower
	}
	return ((1 << (h - 1)) - 1) + leftLast
}

func countHeaps(n int) int64 {
	if n <= 1 {
		return 1
	}
	L := leftSize(n)
	R := n - 1 - L
	return C[n-1][L] * countHeaps(L) * countHeaps(R)
}

func main() {
	N := 3
	integers := []int{1, 2, 3}
	_ = integers
	C = make([][]int64, N+1)
	for i := 0; i <= N; i++ {
		C[i] = make([]int64, N+1)
		C[i][0] = 1
		for j := 1; j <= i; j++ {
			var add int64
			if j <= i-1 {
				add = C[i-1][j]
			}
			C[i][j] = C[i-1][j-1] + add
		}
	}
	fmt.Println(countHeaps(N))
}
