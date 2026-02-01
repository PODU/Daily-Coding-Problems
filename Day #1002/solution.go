// Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
// If the next element x <= res (smallest unreachable, init 1) extend to res+x,
// else res is the answer. O(N) time, O(1) space.
package main

import "fmt"

func smallestNonSubsetSum(nums []int) int {
	res := 1
	for _, x := range nums {
		if x > res {
			break
		}
		res += x
	}
	return res
}

func main() {
	fmt.Println(smallestNonSubsetSum([]int{1, 2, 3, 10})) // 7
}
