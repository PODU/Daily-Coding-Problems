// BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	Val   int
	Left  *Node
	Right *Node
}

func minSumLevel(root *Node) (int, int) {
	if root == nil {
		return -1, 0
	}
	q := []*Node{root}
	level, minLevel := 1, 1
	minSum := math.MaxInt64
	for len(q) > 0 {
		sz := len(q)
		sum := 0
		for i := 0; i < sz; i++ {
			cur := q[i]
			sum += cur.Val
			if cur.Left != nil {
				q = append(q, cur.Left)
			}
			if cur.Right != nil {
				q = append(q, cur.Right)
			}
		}
		q = q[sz:]
		if sum < minSum {
			minSum = sum
			minLevel = level
		}
		level++
	}
	return minLevel, minSum
}

func main() {
	// Tree 1
	r1 := &Node{1, &Node{2, &Node{4, nil, nil}, &Node{5, nil, nil}}, &Node{3, nil, &Node{6, nil, nil}}}
	lvl, sm := minSumLevel(r1)
	fmt.Printf("Level with min sum: %d (sum=%d)\n", lvl, sm)

	// Tree 2
	r2 := &Node{10, &Node{2, nil, nil}, &Node{3, nil, nil}}
	lvl, sm = minSumLevel(r2)
	fmt.Printf("Level with min sum: %d (sum=%d)\n", lvl, sm)
}
