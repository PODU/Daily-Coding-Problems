// H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.
package main

import "fmt"

func hIndex(citations []int) int {
	n := len(citations)
	bucket := make([]int, n+1)
	for _, c := range citations {
		if c > n {
			c = n
		}
		bucket[c]++
	}
	count := 0
	for h := n; h >= 0; h-- {
		count += bucket[h]
		if count >= h {
			return h
		}
	}
	return 0
}

func main() {
	citations := []int{4, 3, 0, 1, 5}
	fmt.Println(hIndex(citations))
}
