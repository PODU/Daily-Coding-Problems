// Day 1296: Smallest positive integer not expressible as a subset sum of a sorted array.
// Greedy: track reachable range [1..r]; if next a[i] <= r+1 extend, else answer r+1. O(N) time.
package main

import "fmt"

func smallestNonSubsetSum(a []int64) int64 {
	var r int64 = 0
	for _, x := range a {
		if x > r+1 {
			break
		}
		r += x
	}
	return r + 1
}

func main() {
	fmt.Println(smallestNonSubsetSum([]int64{1, 2, 3, 10})) // 7
}
