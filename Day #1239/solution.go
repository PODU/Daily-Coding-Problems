// Integer division without * / %. Subtract doubled divisor. Time O(log^2 q).
package main

import "fmt"

func divide(dividend, divisor int) int {
	quotient := 0
	for dividend >= divisor {
		temp, multiple := divisor, 1
		for (temp << 1) <= dividend {
			temp <<= 1
			multiple <<= 1
		}
		dividend -= temp
		quotient += multiple
	}
	return quotient
}

func main() {
	fmt.Println(divide(43, 5))
	fmt.Println(divide(100, 10))
}
