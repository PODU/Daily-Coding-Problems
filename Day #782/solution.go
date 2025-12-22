// H-index via bucket counting.
// Bucket papers by citation count (capped at N), scan from high to low accumulating. O(n) time, O(n) space.
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
	total := 0
	for h := n; h >= 0; h-- {
		total += bucket[h]
		if total >= h {
			return h
		}
	}
	return 0
}

func main() {
	citations := []int{4, 3, 0, 1, 5}
	fmt.Println(hIndex(citations))
}
