// Fast (binary) exponentiation by squaring on a 64-bit result.
// Time: O(log y), Space: O(1).
package main

import "fmt"

func fastPow(x, y int64) int64 {
	var result int64 = 1
	for y > 0 {
		if y&1 == 1 {
			result *= x
		}
		x *= x
		y >>= 1
	}
	return result
}

func main() {
	fmt.Println(fastPow(2, 10))
}
