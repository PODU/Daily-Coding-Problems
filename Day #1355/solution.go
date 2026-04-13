// Longest run of consecutive 1s in binary. Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(run) time, O(1) space.
package main

import "fmt"

func longestRun(n uint) int {
	count := 0
	for n != 0 {
		n &= (n << 1)
		count++
	}
	return count
}

func main() {
	fmt.Println(longestRun(156))
}
