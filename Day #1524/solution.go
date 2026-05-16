// First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
// Time O(n), Space O(1) extra (in-place).
package main

import "fmt"

func firstMissingPositive(arr []int) int {
	a := make([]int, len(arr))
	copy(a, arr)
	n := len(a)
	for i := 0; i < n; i++ {
		for a[i] >= 1 && a[i] <= n && a[a[i]-1] != a[i] {
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
