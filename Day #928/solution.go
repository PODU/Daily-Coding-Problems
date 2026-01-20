// Iterate integers, sum digits, count until the n-th whose digit sum is 10.
// Time O(answer * digits), Space O(1).
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
	count, num := 0, 0
	for count < n {
		num++
		if digitSum(num) == 10 {
			count++
		}
	}
	return num
}

func main() {
	fmt.Println(nthPerfect(1))
}
