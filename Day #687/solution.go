// 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
// State = [9]int board (blank=0); explore by sliding a tile into the blank.
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type Board [9]int

var goal = Board{1, 2, 3, 4, 5, 6, 7, 8, 0}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

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

type Item struct {
	f int
	b Board
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

func neighbors(b Board) []Board {
	z := 0
	for i, v := range b {
		if v == 0 {
			z = i
			break
		}
	}
	zr, zc := z/3, z%3
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	var res []Board
	for _, d := range dirs {
		nr, nc := zr+d[0], zc+d[1]
		if nr >= 0 && nr < 3 && nc >= 0 && nc < 3 {
			nz := nr*3 + nc
			nb := b
			nb[z], nb[nz] = nb[nz], nb[z]
			res = append(res, nb)
		}
	}
	return res
}

func main() {
	start := Board{1, 2, 3, 4, 0, 6, 7, 5, 8}
	g := map[Board]int{start: 0}
	parent := map[Board]Board{}
	pq := &PQ{}
	heap.Init(pq)
	heap.Push(pq, Item{manhattan(start), start})

	for pq.Len() > 0 {
		cur := heap.Pop(pq).(Item)
		if cur.b == goal {
			break
		}
		if cur.f > g[cur.b]+manhattan(cur.b) { // stale entry
			continue
		}
		for _, nb := range neighbors(cur.b) {
			ng := g[cur.b] + 1
			if old, ok := g[nb]; !ok || ng < old {
				g[nb] = ng
				parent[nb] = cur.b
				heap.Push(pq, Item{ng + manhattan(nb), nb})
			}
		}
	}

	var path []Board
	cur := goal
	for {
		path = append([]Board{cur}, path...)
		if cur == start {
			break
		}
		cur = parent[cur]
	}

	moves := len(path) - 1
	fmt.Printf("Solved in %d moves\n", moves)
	fmt.Println("Goal board:")
	gb := path[len(path)-1]
	var sb strings.Builder
	for i, v := range gb {
		sb.WriteString(fmt.Sprintf("%d", v))
		if i%3 == 2 {
			sb.WriteString("\n")
		} else {
			sb.WriteString(" ")
		}
	}
	fmt.Print(sb.String())
}
