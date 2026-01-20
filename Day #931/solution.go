// Day 931: GCD of n numbers by folding pairwise gcd.
// Time: O(n * log(maxVal)), Space: O(1).
package main

import "fmt"

func gcd(a, b int64) int64 {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func gcdList(nums []int64) int64 {
	var g int64 = 0
	for _, x := range nums {
		g = gcd(g, x)
	}
	return g
}

func main() {
	fmt.Println(gcdList([]int64{42, 56, 14})) // 14
}
