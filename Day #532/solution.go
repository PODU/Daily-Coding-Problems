// Count attacking bishop pairs: group by diagonals r+c and r-c, sum C(size,2).
// Time O(N), Space O(N).
package main

import "fmt"

func countAttackingPairs(bishops [][2]int) int64 {
	diag1 := map[int]int64{}
	diag2 := map[int]int64{}
	for _, b := range bishops {
		diag1[b[0]+b[1]]++
		diag2[b[0]-b[1]]++
	}
	var pairs int64
	for _, c := range diag1 {
		pairs += c * (c - 1) / 2
	}
	for _, c := range diag2 {
		pairs += c * (c - 1) / 2
	}
	return pairs
}

func main() {
	bishops := [][2]int{{0, 0}, {1, 2}, {2, 2}, {4, 0}}
	fmt.Println(countAttackingPairs(bishops))
}
