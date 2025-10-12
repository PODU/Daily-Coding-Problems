// Day 420: n-th positive integer whose digits sum to exactly 10.
// Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).
package main

import "fmt"

func digitSum(x int) int {
	s := 0
	for x > 0 {
		s += x % 10
		x /= 10
	}
	return s
}

func nthPerfect(n int) int {
	count := 0
	x := 0
	for count < n {
		x++
		if digitSum(x) == 10 {
			count++
		}
	}
	return x
}

func main() {
	fmt.Println(nthPerfect(1)) // 19
	fmt.Println(nthPerfect(2)) // 28
}
