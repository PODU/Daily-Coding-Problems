// Day 1448: Fewest bricks cut by a vertical line through a brick wall.
// Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
// Time O(total bricks), Space O(distinct edges).
package main

import "fmt"

func leastBricks(wall [][]int) int {
	edges := map[int]int{}
	best := 0
	for _, row := range wall {
		pos := 0
		for i := 0; i+1 < len(row); i++ { // skip far right edge
			pos += row[i]
			edges[pos]++
			if edges[pos] > best {
				best = edges[pos]
			}
		}
	}
	return len(wall) - best
}

func main() {
	wall := [][]int{
		{3, 5, 1, 1},
		{2, 3, 3, 2},
		{5, 5},
		{4, 4, 2},
		{1, 3, 3, 3},
		{1, 1, 6, 1, 1},
	}
	fmt.Println(leastBricks(wall)) // 2
}
