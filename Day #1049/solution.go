// LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
// Avoids a 1e9-size counting array; memory scales with N, not value range. Else: external merge sort.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func radixSort(a []uint32) []uint32 {
	buf := make([]uint32, len(a))
	for shift := uint(0); shift < 32; shift += 8 {
		var count [256]int
		for _, v := range a {
			count[(v>>shift)&0xFF]++
		}
		sum := 0
		for i := 0; i < 256; i++ {
			c := count[i]
			count[i] = sum
			sum += c
		}
		for _, v := range a {
			d := (v >> shift) & 0xFF
			buf[count[d]] = v
			count[d]++
		}
		a, buf = buf, a
	}
	return a
}

func main() {
	a := []uint32{829, 3, 1000000000, 42, 17, 999, 256, 0, 524287, 42}
	a = radixSort(a)
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = strconv.FormatUint(uint64(v), 10)
	}
	fmt.Println(strings.Join(parts, " "))
}
