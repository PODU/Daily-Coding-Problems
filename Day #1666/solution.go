// Day 1666: GCD of n numbers.
// Approach: fold Euclid's algorithm across the list. Time O(n*log(max)), Space O(1).
package main

import "fmt"

func gcd2(a, b int) int {
	if a < 0 {
		a = -a
	}
	if b < 0 {
		b = -b
	}
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func gcdList(nums []int) int {
	g := 0
	for _, x := range nums {
		g = gcd2(g, x)
	}
	return g
}

func main() {
	fmt.Println(gcdList([]int{42, 56, 14})) // 14
}
