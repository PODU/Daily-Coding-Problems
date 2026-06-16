// Day 1678: Integer division without / * %. Subtract largest shifted multiple of
// divisor each round (doubling). Time O(log(quotient)), Space O(1).
package main

import "fmt"

func divide(a, b int) int {
	q := 0
	for a >= b {
		temp, mult := b, 1
		for a >= temp<<1 {
			temp <<= 1
			mult <<= 1
		}
		a -= temp
		q += mult
	}
	return q
}

func main() {
	fmt.Println(divide(43, 8)) // 5
}
