// Bishops attack along diagonals: group by (row-col) and (row+col); each group of k adds k*(k-1)/2. O(N) time, O(N) space.
package main

import "fmt"

func countAttackingPairs(M int, bishops [][2]int) int64 {
	diag := make(map[int]int64)
	anti := make(map[int]int64)
	for _, b := range bishops {
		diag[b[0]-b[1]]++
		anti[b[0]+b[1]]++
	}
	var pairs int64
	for _, k := range diag {
		pairs += k * (k - 1) / 2
	}
	for _, k := range anti {
		pairs += k * (k - 1) / 2
	}
	return pairs
}

func main() {
	M := 5
	bishops := [][2]int{{0, 0}, {1, 2}, {2, 2}, {4, 0}}
	fmt.Println(countAttackingPairs(M, bishops))
}
