// Integer division without '/': repeated doubling/subtraction.
// Time: O(log n), Space: O(1).
package main

import "fmt"

func absL(x int64) int64 {
	if x < 0 {
		return -x
	}
	return x
}

func divide(a, b int64) (int64, int64) {
	neg := (a < 0) != (b < 0)
	x, y := absL(a), absL(b)
	var q int64 = 0
	for x >= y {
		temp, mult := y, int64(1)
		for x >= (temp << 1) {
			temp <<= 1
			mult <<= 1
		}
		x -= temp
		q += mult
	}
	if neg {
		q = -q
	}
	return q, x
}

func main() {
	q, r := divide(10, 3)
	fmt.Printf("(%d, %d)\n", q, r)
}
