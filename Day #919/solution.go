// Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
// Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
package main

import "fmt"

func leastBricks(wall [][]int) int {
	freq := make(map[int]int)
	best := 0
	for _, row := range wall {
		sum := 0
		for i := 0; i+1 < len(row); i++ {
			sum += row[i]
			freq[sum]++
			if freq[sum] > best {
				best = freq[sum]
			}
		}
	}
	return len(wall) - best
}

func main() {
	wall := [][]int{{3, 5, 1, 1}, {2, 3, 3, 2}, {5, 5}, {4, 4, 2}, {1, 3, 3, 3}, {1, 1, 6, 1, 1}}
	fmt.Println(leastBricks(wall))
}
