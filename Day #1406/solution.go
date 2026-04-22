// BFS level-order: sum each level, track the level (1-indexed) with min sum.
// Time O(n), Space O(width).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val  int
	l, r *Node
}

func minSumLevel(root *Node) (int, int) {
	if root == nil {
		return -1, 0
	}
	q := []*Node{root}
	level, bestLevel := 0, 1
	bestSum := math.MaxInt64
	for len(q) > 0 {
		var next []*Node
		sum := 0
		level++
		for _, n := range q {
			sum += n.val
			if n.l != nil {
				next = append(next, n.l)
			}
			if n.r != nil {
				next = append(next, n.r)
			}
		}
		if sum < bestSum {
			bestSum = sum
			bestLevel = level
		}
		q = next
	}
	return bestLevel, bestSum
}

func main() {
	root := &Node{10,
		&Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}},
		&Node{3, nil, nil}}
	lvl, sum := minSumLevel(root)
	fmt.Printf("Level with minimum sum: %d (sum = %d)\n", lvl, sum)
}
