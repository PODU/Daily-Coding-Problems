// Largest product of any three integers.
// Sort once; answer is max of top-3 product and (two smallest * largest).
// Time O(n log n), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func largestProductOfThree(a []int) int {
	sort.Ints(a)
	n := len(a)
	p1 := a[n-1] * a[n-2] * a[n-3]
	p2 := a[0] * a[1] * a[n-1]
	if p1 > p2 {
		return p1
	}
	return p2
}

func main() {
	fmt.Println(largestProductOfThree([]int{-10, -10, 5, 2}))
}
