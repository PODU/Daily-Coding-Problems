// Day 940: Time for a message from node 0 to reach all = max shortest-path distance (Dijkstra).
// Time O(E log V), Space O(V + E). Returns -1 if some node is unreachable.
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
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	it := old[n-1]
	*p = old[:n-1]
	return it
}

func networkDelay(n int, edges [][3]int, src int) int {
	const INF = int(1e18)
	adj := make([][][2]int, n+1)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], [2]int{e[1], e[2]})
	}
	dist := make([]int, n+1)
	for i := range dist {
		dist[i] = INF
	}
	dist[src] = 0
	h := &pq{{0, src}}
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		if cur.dist > dist[cur.node] {
			continue
		}
		for _, nb := range adj[cur.node] {
			v, w := nb[0], nb[1]
			if cur.dist+w < dist[v] {
				dist[v] = cur.dist + w
				heap.Push(h, item{dist[v], v})
			}
		}
	}
	ans := 0
	for _, d := range dist {
		if d == INF {
			return -1
		}
		if d > ans {
			ans = d
		}
	}
	return ans
}

func main() {
	edges := [][3]int{
		{0, 1, 5}, {0, 2, 3}, {0, 5, 4},
		{1, 3, 8}, {2, 3, 1}, {3, 5, 10}, {3, 4, 5},
	}
	fmt.Println(networkDelay(5, edges, 0)) // 9
}
