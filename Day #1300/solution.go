// Day 1300: Find a contiguous subarray summing to K (handles negatives).
// Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.
package main

import "fmt"

func subarraySum(a []int, K int) []int {
	firstIndex := map[int]int{0: -1}
	prefix := 0
	for j, v := range a {
		prefix += v
		if i, ok := firstIndex[prefix-K]; ok {
			return a[i+1 : j+1]
		}
		if _, ok := firstIndex[prefix]; !ok {
			firstIndex[prefix] = j
		}
	}
	return []int{}
}

func main() {
	fmt.Println(subarraySum([]int{1, 2, 3, 4, 5}, 9)) // [2 3 4]
}
