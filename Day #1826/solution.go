// Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
// O((V+E) log V).
package main

import (
	"container/heap"
	"fmt"
)

type item struct{ dist, node int }
type pq []item

func (p pq) Len() int            { return len(p) }
func (p pq) Less(i, j int) bool  { return p[i].dist < p[j].dist }
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{}   { old := *p; n := len(old); x := old[n-1]; *p = old[:n-1]; return x }

func main() {
	edges := [][3]int{{0, 1, 5}, {0, 2, 3}, {0, 5, 4}, {1, 3, 8}, {2, 3, 1}, {3, 5, 10}, {3, 4, 5}}
	maxNode := 0
	for _, e := range edges {
		if e[0] > maxNode {
			maxNode = e[0]
		}
		if e[1] > maxNode {
			maxNode = e[1]
		}
	}
	V := maxNode + 1
	adj := make([][][2]int, V)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], [2]int{e[1], e[2]})
		adj[e[1]] = append(adj[e[1]], [2]int{e[0], e[2]})
	}

	const INF = int(1e18)
	dist := make([]int, V)
	for i := range dist {
		dist[i] = INF
	}
	dist[0] = 0
	q := &pq{{0, 0}}
	for q.Len() > 0 {
		it := heap.Pop(q).(item)
		if it.dist > dist[it.node] {
			continue
		}
		for _, e := range adj[it.node] {
			v, w := e[0], e[1]
			if it.dist+w < dist[v] {
				dist[v] = it.dist + w
				heap.Push(q, item{dist[v], v})
			}
		}
	}
	ans := 0
	for _, d := range dist {
		if d > ans {
			ans = d
		}
	}
	fmt.Println(ans) // 9
}
