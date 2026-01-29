// Longest consecutive run of 1-bits via the bit trick: n &= (n>>1) shrinks every
// run by one each step; iterations until n==0 equals the longest run length.
// Time: O(longest run), Space: O(1).
package main

import "fmt"

func longestRun(n uint) int {
	count := 0
	for n != 0 {
		count++
		n &= (n >> 1)
	}
	return count
}

func main() {
	fmt.Println(longestRun(156)) // 156 = 10011100 -> 3
}
