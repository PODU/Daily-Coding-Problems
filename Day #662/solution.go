// Day 662: GCD of n numbers via repeated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
package main

import "fmt"

func gcd2(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func gcdAll(v []int) int {
	g := 0
	for _, x := range v {
		g = gcd2(g, x)
	}
	return g
}

func main() {
	fmt.Println(gcdAll([]int{42, 56, 14})) // 14
}
