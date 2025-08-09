// Day 88: Integer division using subtraction of shifted divisor (doubling).
// Time O(log^2 q), Space O(1).
package main

import "fmt"

func divide(dividend, divisor int) int {
	quotient := 0
	for dividend >= divisor {
		temp, multiple := divisor, 1
		for dividend >= (temp << 1) {
			temp <<= 1
			multiple <<= 1
		}
		dividend -= temp
		quotient += multiple
	}
	return quotient
}

func main() {
	fmt.Println(divide(10, 3)) // 3
	fmt.Println(divide(27, 4)) // 6
}
