// Day 413: Ordered ways to climb a staircase with allowed step sizes X.
// DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
package main

import "fmt"

func countWays(n int, steps []int) int64 {
	ways := make([]int64, n+1)
	ways[0] = 1
	for i := 1; i <= n; i++ {
		for _, x := range steps {
			if x <= i {
				ways[i] += ways[i-x]
			}
		}
	}
	return ways[n]
}

func main() {
	fmt.Println(countWays(4, []int{1, 2}))      // 5
	fmt.Println(countWays(10, []int{1, 3, 5}))  // generalized X={1,3,5}
}
