// Day 694: First missing positive integer in O(n) time, O(1) space.
// Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
// Time O(n), Space O(1) (in-place).
package main

import "fmt"

func firstMissingPositive(a []int) int {
	n := len(a)
	for i := 0; i < n; i++ {
		for a[i] > 0 && a[i] <= n && a[a[i]-1] != a[i] {
			j := a[i] - 1
			a[i], a[j] = a[j], a[i]
		}
	}
	for i := 0; i < n; i++ {
		if a[i] != i+1 {
			return i + 1
		}
	}
	return n + 1
}

func main() {
	fmt.Println(firstMissingPositive([]int{3, 4, -1, 1})) // 2
	fmt.Println(firstMissingPositive([]int{1, 2, 0}))     // 3
}
