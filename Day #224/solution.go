// Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
// Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
// Time O(N), Space O(1).
package main

import "fmt"

func smallestNonSubsetSum(a []int64) int64 {
	var ans int64 = 1 // smallest unreachable so far
	for _, x := range a {
		if x > ans {
			break
		}
		ans += x
	}
	return ans
}

func main() {
	fmt.Println(smallestNonSubsetSum([]int64{1, 2, 3, 10})) // 7
}
