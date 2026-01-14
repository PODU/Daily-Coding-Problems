// Integer division without / * %: subtract largest shifted multiple of divisor.
// Bit-shifting. Time O((log n)^2), Space O(1).
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
	fmt.Println(divide(43, 8))
	fmt.Println(divide(100, 9))
}
