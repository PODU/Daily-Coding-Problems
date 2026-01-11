// 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
// State = 9-char string, 0 = blank. Time/space depend on solution depth.
package main

import (
	"container/heap"
	"fmt"
)

const goal = "123456780"

func manhattan(s string) int {
	d := 0
	for i := 0; i < 9; i++ {
		if s[i] == '0' {
			continue
		}
		v := int(s[i] - '1')
		d += abs(i/3-v/3) + abs(i%3-v%3)
	}
	return d
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

type item struct {
	f, g  int
	state string
}
type pq []item

func (p pq) Len() int            { return len(p) }
func (p pq) Less(i, j int) bool  { return p[i].f < p[j].f }
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func solve(start string) []int {
	open := &pq{{manhattan(start), 0, start}}
	heap.Init(open)
	g := map[string]int{start: 0}
	parentState := map[string]string{}
	parentTile := map[string]int{}
	dr := []int{-1, 1, 0, 0}
	dc := []int{0, 0, -1, 1}
	for open.Len() > 0 {
		cur := heap.Pop(open).(item)
		if cur.state == goal {
			break
		}
		if cur.g > g[cur.state] {
			continue
		}
		z := -1
		for i := 0; i < 9; i++ {
			if cur.state[i] == '0' {
				z = i
				break
			}
		}
		r, c := z/3, z%3
		for k := 0; k < 4; k++ {
			nr, nc := r+dr[k], c+dc[k]
			if nr < 0 || nr > 2 || nc < 0 || nc > 2 {
				continue
			}
			nz := nr*3 + nc
			b := []byte(cur.state)
			b[z], b[nz] = b[nz], b[z]
			nxt := string(b)
			ng := cur.g + 1
			if old, ok := g[nxt]; !ok || ng < old {
				g[nxt] = ng
				parentState[nxt] = cur.state
				parentTile[nxt] = int(cur.state[nz] - '0')
				heap.Push(open, item{ng + manhattan(nxt), ng, nxt})
			}
		}
	}
	moves := []int{}
	cur := goal
	for cur != start {
		moves = append([]int{parentTile[cur]}, moves...)
		cur = parentState[cur]
	}
	return moves
}

func main() {
	start := "123406758" // [[1,2,3],[4,_,6],[7,5,8]]
	moves := solve(start)
	fmt.Printf("Solved in %d moves: %v\n", len(moves), moves)
}
