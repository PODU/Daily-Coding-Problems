// Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
package main

import "fmt"

func sortedSquares(a []int) []int {
	n := len(a)
	res := make([]int, n)
	lo, hi := 0, n-1
	for k := n - 1; k >= 0; k-- {
		sl, sh := a[lo]*a[lo], a[hi]*a[hi]
		if sl > sh {
			res[k] = sl
			lo++
		} else {
			res[k] = sh
			hi--
		}
	}
	return res
}

func main() {
	fmt.Println(sortedSquares([]int{-9, -2, 0, 2, 3})) // [0 4 4 9 81]
}
