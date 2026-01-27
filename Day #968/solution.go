// Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
// Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).
package main

import "fmt"

func minJumps(n int64) int {
	if n < 0 {
		n = -n
	}
	var k, sum int64
	for sum < n || (sum-n)%2 != 0 {
		k++
		sum += k
	}
	return int(k)
}

func main() {
	fmt.Println(minJumps(10)) // 4  (1+2+3+4=10)
}
