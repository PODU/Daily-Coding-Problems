// Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
// top of each adjacent-ones run, carry up and clear below. Time O(log N).
package main

import "fmt"

func nextSparse(x int64) int64 {
	if x == 0 {
		return 0
	}
	var b []int
	for t := x; t != 0; t >>= 1 {
		b = append(b, int(t&1))
	}
	b = append(b, 0, 0) // padding for carries
	n := len(b)
	for i := 1; i < n-1; i++ {
		if b[i] == 1 && b[i-1] == 1 && b[i+1] == 0 {
			b[i+1] = 1
			for j := i; j >= 0; j-- {
				b[j] = 0
			}
		}
	}
	var ans int64
	for i := 0; i < n; i++ {
		if b[i] == 1 {
			ans |= int64(1) << uint(i)
		}
	}
	return ans
}

func main() {
	fmt.Println(nextSparse(22)) // 32
}
