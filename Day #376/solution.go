// Closest coin by Manhattan distance. Linear scan.
// Time: O(n), Space: O(1).
package main

import "fmt"

func abs(x int) int { if x < 0 { return -x }; return x }

func closestCoin(me [2]int, coins [][2]int) [2]int {
	best := coins[0]
	bestD := 1 << 62
	for _, c := range coins {
		d := abs(c[0]-me[0]) + abs(c[1]-me[1])
		if d < bestD {
			bestD = d
			best = c
		}
	}
	return best
}

func main() {
	me := [2]int{0, 2}
	coins := [][2]int{{0, 4}, {1, 0}, {2, 0}, {3, 2}}
	b := closestCoin(me, coins)
	fmt.Printf("(%d, %d)\n", b[0], b[1])
}
