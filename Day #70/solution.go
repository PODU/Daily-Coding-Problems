// n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
package main

import "fmt"

func digitSum(x int64) int {
	s := 0
	for x > 0 {
		s += int(x % 10)
		x /= 10
	}
	return s
}

func nthPerfect(n int) int64 {
	count := 0
	var num int64 = 0
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
	fmt.Println(nthPerfect(2))
}
