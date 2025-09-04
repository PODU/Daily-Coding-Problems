// Day 214: Longest consecutive run of 1s in binary representation.
// Approach: n &= (n<<1) collapses runs; count iterations. Time O(longest run), Space O(1).
package main

import "fmt"

func longestRun(n uint64) int {
	count := 0
	for n != 0 {
		n &= (n << 1)
		count++
	}
	return count
}

func main() {
	fmt.Println(longestRun(156)) // 156 = 10011100 -> 3
}
