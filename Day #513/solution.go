// Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
package main

import "fmt"

func subarraySum(a []int, K int) []int {
	seen := map[int]int{0: -1}
	pre := 0
	for i, v := range a {
		pre += v
		if start, ok := seen[pre-K]; ok {
			return a[start+1 : i+1]
		}
		if _, ok := seen[pre]; !ok {
			seen[pre] = i
		}
	}
	return []int{}
}

func main() {
	fmt.Println(subarraySum([]int{1, 2, 3, 4, 5}, 9))
}
