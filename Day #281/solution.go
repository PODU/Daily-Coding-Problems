// Day 281: Fewest bricks cut by a vertical line. Count prefix-sum edge positions;
// answer = rows - max edge frequency. Time O(total bricks), Space O(distinct edges).
package main

import "fmt"

func fewestCuts(wall [][]int) int {
	edge := make(map[int]int)
	best := 0
	for _, row := range wall {
		sum := 0
		for i := 0; i+1 < len(row); i++ {
			sum += row[i]
			edge[sum]++
			if edge[sum] > best {
				best = edge[sum]
			}
		}
	}
	return len(wall) - best
}

func main() {
	wall := [][]int{
		{3, 5, 1, 1}, {2, 3, 3, 2}, {5, 5},
		{4, 4, 2}, {1, 3, 3, 3}, {1, 1, 6, 1, 1},
	}
	fmt.Println(fewestCuts(wall)) // 2
}
