// Day 1483: Shortest closed route from home (0) strictly ascending then
// descending. up[v]: shortest ascending 0->v; down[v]: shortest descending v->0
// (Dijkstra from 0 on reversed descending graph). Answer = min up[v]+down[v].
// Time O((V+E) log V), Space O(V+E).
package main

import (
	"container/heap"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type edge struct {
	to int
	w  int64
}

type item struct {
	dist int64
	node int
}
type pq []item

func (p pq) Len() int            { return len(p) }
func (p pq) Less(i, j int) bool  { return p[i].dist < p[j].dist }
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

const INF = math.MaxInt64

func dijkstra(n int, adj [][]edge, src int) ([]int64, []int) {
	dist := make([]int64, n)
	pred := make([]int, n)
	for i := range dist {
		dist[i] = INF
		pred[i] = -1
	}
	dist[src] = 0
	q := &pq{{0, src}}
	for q.Len() > 0 {
		it := heap.Pop(q).(item)
		if it.dist > dist[it.node] {
			continue
		}
		for _, e := range adj[it.node] {
			if it.dist+e.w < dist[e.to] {
				dist[e.to] = it.dist + e.w
				pred[e.to] = it.node
				heap.Push(q, item{dist[e.to], e.to})
			}
		}
	}
	return dist, pred
}

func main() {
	n := 5
	elev := []int{5, 25, 15, 20, 10}
	edges := [][3]int64{
		{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
		{2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}}

	upAdj := make([][]edge, n)
	revDesc := make([][]edge, n)
	for _, e := range edges {
		a, b, w := int(e[0]), int(e[1]), e[2]
		if elev[b] > elev[a] {
			upAdj[a] = append(upAdj[a], edge{b, w})
		} else if elev[b] < elev[a] {
			revDesc[b] = append(revDesc[b], edge{a, w})
		}
	}

	up, upPred := dijkstra(n, upAdj, 0)
	down, downPred := dijkstra(n, revDesc, 0)

	best := int64(INF)
	peak := -1
	for v := 1; v < n; v++ {
		if up[v] != INF && down[v] != INF && up[v] > 0 && down[v] > 0 && up[v]+down[v] < best {
			best = up[v] + down[v]
			peak = v
		}
	}

	var upPath []int
	for c := peak; c != -1; c = upPred[c] {
		upPath = append([]int{c}, upPath...)
	}
	route := append([]int{}, upPath...)
	for c := downPred[peak]; c != -1; c = downPred[c] {
		route = append(route, c)
	}

	parts := make([]string, len(route))
	for i, v := range route {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Printf("The shortest valid path would be %s, with a distance of %d.\n",
		strings.Join(parts, " -> "), best)
	// The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
}
