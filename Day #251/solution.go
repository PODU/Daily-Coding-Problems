// External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
// Real answer for data exceeding RAM: external merge sort (split into chunks ->
// sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
// bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.
package main

import (
	"fmt"
	"sort"
	"strings"
)

func bucketSort(data []int64, maxVal int64, numBuckets int) []int64 {
	width := maxVal/int64(numBuckets) + 1
	buckets := make([][]int64, numBuckets)
	for _, x := range data {
		b := int(x / width)
		if b >= numBuckets {
			b = numBuckets - 1
		}
		buckets[b] = append(buckets[b], x)
	}
	out := make([]int64, 0, len(data))
	for _, bk := range buckets {
		sort.Slice(bk, func(i, j int) bool { return bk[i] < bk[j] }) // bucket fits in memory
		out = append(out, bk...)
	}
	return out
}

func main() {
	input := []int64{5, 1, 4, 2, 8, 1000000000, 0}
	sorted := bucketSort(input, 1000000000, 16)
	parts := make([]string, len(sorted))
	for i, x := range sorted {
		parts[i] = fmt.Sprintf("%d", x)
	}
	fmt.Println(strings.Join(parts, " "))
}
