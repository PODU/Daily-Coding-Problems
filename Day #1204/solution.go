// Day 1204: GCD of n numbers.
// Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
package main

import "fmt"

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func gcdAll(nums []int) int {
	g := 0
	for _, x := range nums {
		g = gcd(g, x)
	}
	return g
}

func main() {
	fmt.Println(gcdAll([]int{42, 56, 14})) // 14
}
