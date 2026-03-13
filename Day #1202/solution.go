// Day 1202: Minimum swaps so couples sit side by side.
// Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).
package main

import "fmt"

func minSwaps(row []int) int {
	n := len(row)
	pos := make([]int, n)
	for i, v := range row {
		pos[v] = i
	}
	swaps := 0
	for i := 0; i < n; i += 2 {
		partner := row[i] ^ 1
		if row[i+1] != partner {
			j := pos[partner]
			pos[row[i+1]], pos[row[j]] = pos[row[j]], pos[row[i+1]]
			row[i+1], row[j] = row[j], row[i+1]
			swaps++
		}
	}
	return swaps
}

func main() {
	fmt.Println(minSwaps([]int{0, 2, 1, 3})) // 1
}
