// Day 375: h-index via counting sort.
// Bucket citations (capped at n), then scan h from n down accumulating papers
// with >= h citations; first h with count >= h wins. Time O(n), Space O(n).
package main

import "fmt"

func hIndex(cites []int) int {
	n := len(cites)
	buckets := make([]int, n+1)
	for _, c := range cites {
		if c > n {
			c = n
		}
		buckets[c]++
	}
	total := 0
	for h := n; h >= 0; h-- {
		total += buckets[h]
		if total >= h {
			return h
		}
	}
	return 0
}

func main() {
	fmt.Println(hIndex([]int{4, 0, 0, 2, 3})) // 2
}
