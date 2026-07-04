// Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
// (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
// 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
// If even the million don't fit, fall back to external merge sort (chunk-sort to
// disk, then k-way merge).
package main

import (
	"fmt"
	"strings"
)

func radixSort(a []uint32) []uint32 {
	a = append([]uint32(nil), a...)
	tmp := make([]uint32, len(a))
	for shift := uint(0); shift < 32; shift += 8 {
		var count [256]int
		for _, x := range a {
			count[(x>>shift)&0xFF]++
		}
		sum := 0
		for i := 0; i < 256; i++ {
			c := count[i]
			count[i] = sum
			sum += c
		}
		for _, x := range a {
			d := (x >> shift) & 0xFF
			tmp[count[d]] = x
			count[d]++
		}
		a, tmp = tmp, a
	}
	return a
}

func main() {
	data := []uint32{999999999, 0, 123456789, 42, 1000000000, 7, 500000000}
	sorted := radixSort(data)
	parts := make([]string, len(sorted))
	for i, x := range sorted {
		parts[i] = fmt.Sprintf("%d", x)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
