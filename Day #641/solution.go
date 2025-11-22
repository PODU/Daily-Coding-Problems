// Day 641: Smallest positive integer not expressible as a subset sum.
// Approach: scan sorted array; if a[i] > reach+1 a gap exists, else reach += a[i].
// Time: O(N), Space: O(1).
package main

import "fmt"

func smallestNonSum(a []int64) int64 {
	var reach int64 = 0 // all of [1..reach] are representable
	for _, x := range a {
		if x > reach+1 {
			break
		}
		reach += x
	}
	return reach + 1
}

func main() {
	fmt.Println(smallestNonSum([]int64{1, 2, 3, 10})) // 7
}
