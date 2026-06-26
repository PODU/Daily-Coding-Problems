// h-index via counting sort: bucket citations capped at N, scan from high to low
// accumulating papers until count >= citation level. Time O(N), Space O(N).
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
	acc := 0
	for h := n; h >= 0; h-- {
		acc += bucket[h]
		if acc >= h {
			return h
		}
	}
	return 0
}

func main() {
	citations := []int{4, 3, 0, 1, 5}
	fmt.Println(hIndex(citations))
}
