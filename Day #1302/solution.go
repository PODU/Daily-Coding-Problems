// Day 1302: First missing positive integer in O(n) time, O(1) extra space.
// Cyclic placement: put value v at index v-1; first index i with a[i]!=i+1 gives answer.
package main

import "fmt"

func firstMissingPositive(a []int) int {
	n := len(a)
	for i := 0; i < n; i++ {
		for a[i] > 0 && a[i] <= n && a[a[i]-1] != a[i] {
			a[i], a[a[i]-1] = a[a[i]-1], a[i]
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
