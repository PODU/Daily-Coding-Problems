// Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
// Time: O(E log V), Space: O(V + E).
package main

import (
	"container/heap"
	"fmt"
)

type Item struct {
	dist, node int
}
type PQ []Item

func (p PQ) Len() int            { return len(p) }
func (p PQ) Less(i, j int) bool  { return p[i].dist < p[j].dist }
func (p PQ) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x interface{}) { *p = append(*p, x.(Item)) }
func (p *PQ) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func networkDelay(N int, edges [][3]int) int {
	V := N + 1
	adj := make([][][2]int, V)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], [2]int{e[1], e[2]})
		adj[e[1]] = append(adj[e[1]], [2]int{e[0], e[2]})
	}
	const INF = int(1 << 60)
	dist := make([]int, V)
	for i := range dist {
		dist[i] = INF
	}
	dist[0] = 0
	pq := &PQ{{0, 0}}
	for pq.Len() > 0 {
		cur := heap.Pop(pq).(Item)
		if cur.dist > dist[cur.node] {
			continue
		}
		for _, nx := range adj[cur.node] {
			v, w := nx[0], nx[1]
			if dist[cur.node]+w < dist[v] {
				dist[v] = dist[cur.node] + w
				heap.Push(pq, Item{dist[v], v})
			}
		}
	}
	ans := 0
	for _, d := range dist {
		if d != INF && d > ans {
			ans = d
		}
	}
	return ans
}

func main() {
	N := 5
	edges := [][3]int{{0, 1, 5}, {0, 2, 3}, {0, 5, 4}, {1, 3, 8}, {2, 3, 1}, {3, 5, 10}, {3, 4, 5}}
	fmt.Println(networkDelay(N, edges))
}
