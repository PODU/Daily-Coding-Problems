// Day 1730: Fast integer exponentiation (exponentiation by squaring).
// Square the base and halve the exponent each step. Time: O(log y). Space: O(1).
package main

import "fmt"

func fastPow(x, y int64) int64 {
	if y < 0 { // x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
		inv := fastPow(x, -y)
		if inv == 0 {
			return 0
		}
		return 1 / inv
	}
	var result, base int64 = 1, x
	for y > 0 {
		if y&1 == 1 {
			result *= base
		}
		base *= base
		y >>= 1
	}
	return result
}

func main() {
	fmt.Println(fastPow(2, 10)) // 1024
}
