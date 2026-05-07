// Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
// Index by element count, not by value. LSD radix sort (base 256) is O(N).
// For out-of-core data the same idea generalizes to external merge sort.
// Time O(N) (4 passes for 32-bit), Space O(N).
package main

import "fmt"

func radixSort(in []uint32) []uint32 {
	if len(in) == 0 {
		return in
	}
	out := append([]uint32{}, in...)
	tmp := make([]uint32, len(out))
	for shift := uint(0); shift < 32; shift += 8 {
		var count [257]int
		for _, v := range out {
			count[((v>>shift)&0xFF)+1]++
		}
		for i := 0; i < 256; i++ {
			count[i+1] += count[i]
		}
		for _, v := range out {
			d := (v >> shift) & 0xFF
			tmp[count[d]] = v
			count[d]++
		}
		out, tmp = tmp, out
	}
	return out
}

func main() {
	fmt.Println(radixSort([]uint32{9, 11, 8, 5, 7, 10})) // [5 7 8 9 10 11]
}
