// Day 1287: Minimum bonuses so each employee out-earns lower-scoring neighbors.
// Two passes (left->right, right->left), take max. Time O(n), Space O(n).
package main

import "fmt"

func bonuses(a []int) []int {
	n := len(a)
	b := make([]int, n)
	for i := range b {
		b[i] = 1
	}
	for i := 1; i < n; i++ {
		if a[i] > a[i-1] {
			b[i] = b[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if a[i] > a[i+1] && b[i+1]+1 > b[i] {
			b[i] = b[i+1] + 1
		}
	}
	return b
}

func main() {
	fmt.Println(bonuses([]int{10, 40, 200, 1000, 60, 30})) // [1 2 3 4 2 1]
}
