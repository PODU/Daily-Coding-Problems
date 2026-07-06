// Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
// Time: O(N), Space: O(N).
package main

import "fmt"

func main() {
	M := 5
	_ = M
	bishops := [][2]int{{0, 0}, {1, 2}, {2, 2}, {4, 0}}
	diag := map[int]int64{}
	anti := map[int]int64{}
	for _, b := range bishops {
		diag[b[0]-b[1]]++
		anti[b[0]+b[1]]++
	}
	var pairs int64
	for _, c := range diag {
		pairs += c * (c - 1) / 2
	}
	for _, c := range anti {
		pairs += c * (c - 1) / 2
	}
	fmt.Println(pairs)
}
