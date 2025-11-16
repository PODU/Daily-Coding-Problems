// Day 610: Integer division of positive ints without / , * , or %.
// Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).
package main

import "fmt"

func divide(dividend, divisor int64) int64 {
	var q int64 = 0
	for dividend >= divisor {
		temp, mult := divisor, int64(1)
		for dividend >= (temp << 1) {
			temp <<= 1
			mult <<= 1
		}
		dividend -= temp
		q += mult
	}
	return q
}

func main() {
	fmt.Println(divide(10, 3)) // 3
	fmt.Println(divide(43, 8)) // 5
}
