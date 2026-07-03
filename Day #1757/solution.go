// Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
// Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.
package main

import "fmt"

func climbWays(n int, steps []int) int64 {
	ways := make([]int64, n+1)
	ways[0] = 1
	for i := 1; i <= n; i++ {
		for _, x := range steps {
			if i-x >= 0 {
				ways[i] += ways[i-x]
			}
		}
	}
	return ways[n]
}

func main() {
	N := 4
	fmt.Println(climbWays(N, []int{1, 2})) // 5
	fmt.Println("Generalized X={1,3,5}, N=4:", climbWays(N, []int{1, 3, 5}))
}
