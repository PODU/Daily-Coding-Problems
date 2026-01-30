// Day 990: Count ordered ways to climb N steps using step sizes from set X.
// Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.
package main

import "fmt"

func climbWays(n int, X []int) int64 {
	ways := make([]int64, n+1)
	ways[0] = 1
	for i := 1; i <= n; i++ {
		for _, x := range X {
			if i-x >= 0 {
				ways[i] += ways[i-x]
			}
		}
	}
	return ways[n]
}

func main() {
	fmt.Println("N=4, X={1,2}:", climbWays(4, []int{1, 2}))      // expected 5
	fmt.Println("N=4, X={1,3,5}:", climbWays(4, []int{1, 3, 5})) // generalized
}
