// Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.
package main

import "fmt"

// returns x^y; negative y returns a float64
func ipow(x, y int64) float64 {
	if y < 0 {
		return 1.0 / ipow(x, -y)
	}
	var result int64 = 1
	base := x
	for y > 0 {
		if y&1 == 1 {
			result *= base
		}
		base *= base
		y >>= 1
	}
	return float64(result)
}

func main() {
	fmt.Println(int64(ipow(2, 10))) // 1024
	fmt.Println(int64(ipow(3, 5)))  // 243
	fmt.Println(ipow(2, -2))        // 0.25
}
