// Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
// For each group of size k, add k*(k-1)/2. Time O(N), Space O(N).
package main

import "fmt"

func countAttackingPairs(bishops [][2]int) int64 {
	diag := map[int]int{}
	anti := map[int]int{}
	for _, b := range bishops {
		diag[b[0]-b[1]]++
		anti[b[0]+b[1]]++
	}
	var pairs int64
	for _, k := range diag {
		pairs += int64(k) * int64(k-1) / 2
	}
	for _, k := range anti {
		pairs += int64(k) * int64(k-1) / 2
	}
	return pairs
}

func main() {
	bishops := [][2]int{{0, 0}, {1, 2}, {2, 2}, {4, 0}}
	fmt.Println(countAttackingPairs(bishops)) // 2
}
