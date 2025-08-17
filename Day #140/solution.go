// XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
// Time O(n); Space O(1).
package main

import "fmt"

func twoSingles(nums []int) (int, int) {
	x := 0
	for _, v := range nums {
		x ^= v
	}
	bit := x & (-x) // lowest set bit where the two singles differ
	a, b := 0, 0
	for _, v := range nums {
		if v&bit != 0 {
			a ^= v
		} else {
			b ^= v
		}
	}
	if a > b {
		a, b = b, a
	}
	return a, b
}

func main() {
	a, b := twoSingles([]int{2, 4, 6, 8, 10, 2, 6, 10})
	fmt.Printf("%d and %d\n", a, b) // 4 and 8
}
