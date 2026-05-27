// Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
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
	fmt.Println(longestRun(156)) // 10011100 -> 3
}
