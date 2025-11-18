// Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
// Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
package main

import "fmt"

func leastBricks(wall [][]int) int {
	edges := make(map[int]int)
	maxCount := 0
	for _, row := range wall {
		sum := 0
		for i := 0; i+1 < len(row); i++ {
			sum += row[i]
			edges[sum]++
			if edges[sum] > maxCount {
				maxCount = edges[sum]
			}
		}
	}
	return len(wall) - maxCount
}

func main() {
	wall := [][]int{{3, 5, 1, 1}, {2, 3, 3, 2}, {5, 5}, {4, 4, 2}, {1, 3, 3, 3}, {1, 1, 6, 1, 1}}
	fmt.Println(leastBricks(wall))
}
