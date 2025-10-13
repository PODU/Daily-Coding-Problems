// Day 421: LSD radix sort (base-256). O(n*w) time, O(n) space.
// 4 byte-passes for 32-bit ints; avoids a billion-element counting array.
package main

import "fmt"

func radixSort(a []uint32) []uint32 {
	n := len(a)
	buf := make([]uint32, n)
	for shift := uint(0); shift < 32; shift += 8 {
		var cnt [257]int
		for _, x := range a {
			cnt[((x>>shift)&0xFF)+1]++
		}
		for i := 0; i < 256; i++ {
			cnt[i+1] += cnt[i]
		}
		for _, x := range a {
			d := (x >> shift) & 0xFF
			buf[cnt[d]] = x
			cnt[d]++
		}
		a, buf = buf, a
	}
	return a
}

func main() {
	a := []uint32{5, 3, 1000000000, 0, 42, 7, 42}
	a = radixSort(a)
	fmt.Print("Sorted: [")
	for i, x := range a {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(x)
	}
	fmt.Println("]")
}
