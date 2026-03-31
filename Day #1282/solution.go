// Day 1282: n-th positive integer whose digits sum to 10.
// Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
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
	x, count := 0, 0
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
