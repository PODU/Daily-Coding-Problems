// 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
// Time ~O(b^d) bounded by states explored; Space O(states stored).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type Board [9]int

func manhattan(b Board) int {
	d := 0
	for i, v := range b {
		if v == 0 {
			continue
		}
		gr, gc := (v-1)/3, (v-1)%3
		r, c := i/3, i%3
		d += abs(gr-r) + abs(gc-c)
	}
	return d
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

type Item struct {
	f, g int
	b    Board
}
type PQ []Item

func (p PQ) Len() int            { return len(p) }
func (p PQ) Less(i, j int) bool  { return p[i].f < p[j].f }
func (p PQ) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x interface{}) { *p = append(*p, x.(Item)) }
func (p *PQ) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

type Parent struct {
	b    Board
	move string
}

func main() {
	start := Board{1, 2, 3, 4, 5, 6, 7, 0, 8}
	goal := Board{1, 2, 3, 4, 5, 6, 7, 8, 0}

	moves := []struct {
		delta int
		name  string
	}{{-3, "Up"}, {3, "Down"}, {-1, "Left"}, {1, "Right"}}

	g := map[Board]int{start: 0}
	parent := map[Board]Parent{}
	pq := &PQ{{manhattan(start), 0, start}}
	heap.Init(pq)

	for pq.Len() > 0 {
		it := heap.Pop(pq).(Item)
		cur := it.b
		if it.g > g[cur] {
			continue
		}
		if cur == goal {
			break
		}
		blank := 0
		for i, v := range cur {
			if v == 0 {
				blank = i
			}
		}
		r, c := blank/3, blank%3
		for _, m := range moves {
			if m.name == "Up" && r == 0 {
				continue
			}
			if m.name == "Down" && r == 2 {
				continue
			}
			if m.name == "Left" && c == 0 {
				continue
			}
			if m.name == "Right" && c == 2 {
				continue
			}
			nb := blank + m.delta
			nx := cur
			nx[blank], nx[nb] = nx[nb], nx[blank]
			ng := it.g + 1
			if old, ok := g[nx]; !ok || ng < old {
				g[nx] = ng
				parent[nx] = Parent{cur, m.name}
				heap.Push(pq, Item{ng + manhattan(nx), ng, nx})
			}
		}
	}

	var path []string
	cur := goal
	for cur != start {
		p := parent[cur]
		path = append(path, p.move)
		cur = p.b
	}
	// reverse
	for i, j := 0, len(path)-1; i < j; i, j = i+1, j-1 {
		path[i], path[j] = path[j], path[i]
	}
	fmt.Printf("Solved in %d move(s): %s\n", len(path), strings.Join(path, ", "))
}
