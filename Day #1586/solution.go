// 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
// Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type Board [9]int

const goal = "123456780"

func key(b Board) string {
	var sb strings.Builder
	for _, v := range b {
		sb.WriteByte(byte('0' + v))
	}
	return sb.String()
}

func manhattan(b Board) int {
	d := 0
	for i, v := range b {
		if v == 0 {
			continue
		}
		gi := v - 1
		dr := i/3 - gi/3
		if dr < 0 {
			dr = -dr
		}
		dc := i%3 - gi%3
		if dc < 0 {
			dc = -dc
		}
		d += dr + dc
	}
	return d
}

type item struct {
	f, g int
	b    Board
}
type PQ []item

func (p PQ) Len() int            { return len(p) }
func (p PQ) Less(i, j int) bool  { return p[i].f < p[j].f }
func (p PQ) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *PQ) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func solve(start Board) []int {
	g := map[string]int{key(start): 0}
	parent := map[string]string{key(start): key(start)}
	boards := map[string]Board{key(start): start}
	moved := map[string]int{}
	pq := &PQ{{manhattan(start), 0, start}}
	heap.Init(pq)
	dr := []int{-1, 1, 0, 0}
	dc := []int{0, 0, -1, 1}
	found := false
	for pq.Len() > 0 {
		it := heap.Pop(pq).(item)
		ck := key(it.b)
		if ck == goal {
			found = true
			break
		}
		if it.g > g[ck] {
			continue
		}
		z := 0
		for it.b[z] != 0 {
			z++
		}
		zr, zc := z/3, z%3
		for k := 0; k < 4; k++ {
			nr, nc := zr+dr[k], zc+dc[k]
			if nr < 0 || nr > 2 || nc < 0 || nc > 2 {
				continue
			}
			nz := nr*3 + nc
			nb := it.b
			tile := nb[nz]
			nb[z] = tile
			nb[nz] = 0
			ng := it.g + 1
			nk := key(nb)
			if old, ok := g[nk]; !ok || ng < old {
				g[nk] = ng
				parent[nk] = ck
				boards[nk] = nb
				moved[nk] = tile
				heap.Push(pq, item{ng + manhattan(nb), ng, nb})
			}
		}
	}
	var tiles []int
	if found {
		cur := goal
		sk := key(start)
		for cur != sk {
			tiles = append(tiles, moved[cur])
			cur = parent[cur]
		}
		for i, j := 0, len(tiles)-1; i < j; i, j = i+1, j-1 {
			tiles[i], tiles[j] = tiles[j], tiles[i]
		}
	}
	return tiles
}

func main() {
	start := Board{1, 2, 3, 4, 5, 6, 0, 7, 8}
	tiles := solve(start)
	fmt.Printf("Solved in %d moves\n", len(tiles))
	parts := make([]string, len(tiles))
	for i, t := range tiles {
		parts[i] = fmt.Sprintf("%d", t)
	}
	fmt.Println("Tiles slid: " + strings.Join(parts, ", "))
}
