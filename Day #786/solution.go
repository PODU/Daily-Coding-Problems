// Integer exponentiation by squaring. Time O(log y), Space O(1).
// Demo uses pow(2,10).
package main

import "fmt"

func ipow(x, y int64) int64 {
	result := int64(1)
	base := x
	e := y
	for e > 0 {
		if e&1 == 1 {
			result *= base
		}
		base *= base
		e >>= 1
	}
	return result
}

func main() {
	fmt.Println(ipow(2, 10))
}
