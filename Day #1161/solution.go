// 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
// Time: O(b^d) bounded by states, Space: O(states).
package main

import (
	"container/heap"
	"fmt"
	"strconv"
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
		d += abs(r-gr) + abs(c-gc)
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

func key(b Board) string {
	var sb strings.Builder
	for _, x := range b {
		sb.WriteByte(byte('0' + x))
	}
	return sb.String()
}

func solve(start, goal Board) int {
	pq := &PQ{}
	heap.Init(pq)
	heap.Push(pq, Item{manhattan(start), 0, start})
	best := map[string]int{key(start): 0}
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for pq.Len() > 0 {
		it := heap.Pop(pq).(Item)
		b, g := it.b, it.g
		bk := key(b)
		if b == goal {
			return g
		}
		if g > best[bk] {
			continue
		}
		z := 0
		for b[z] != 0 {
			z++
		}
		r, c := z/3, z%3
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr < 0 || nr > 2 || nc < 0 || nc > 2 {
				continue
			}
			j := nr*3 + nc
			nb := b
			nb[z], nb[j] = nb[j], nb[z]
			ng := g + 1
			nk := key(nb)
			if cur, ok := best[nk]; !ok || ng < cur {
				best[nk] = ng
				heap.Push(pq, Item{ng + manhattan(nb), ng, nb})
			}
		}
	}
	return -1
}

func main() {
	start := Board{1, 2, 3, 4, 0, 6, 7, 5, 8}
	goal := Board{1, 2, 3, 4, 5, 6, 7, 8, 0}
	moves := solve(start, goal)
	fmt.Printf("Solved in %d moves\n", moves)
	for r := 0; r < 3; r++ {
		cells := make([]string, 0, 3)
		for c := 0; c < 3; c++ {
			v := goal[r*3+c]
			if v == 0 {
				cells = append(cells, "_")
			} else {
				cells = append(cells, strconv.Itoa(v))
			}
		}
		fmt.Println(strings.Join(cells, " "))
	}
}
