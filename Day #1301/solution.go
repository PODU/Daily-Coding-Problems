// Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
// Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).
package main

import "fmt"

func minJumps(N int64) int {
	if N < 0 {
		N = -N
	}
	var k, sum int64 = 0, 0
	for sum < N || (sum-N)%2 != 0 {
		k++
		sum += k
	}
	return int(k)
}

func main() {
	fmt.Println(minJumps(3)) // 2
	fmt.Println(minJumps(2)) // 3
}
