// Alien dictionary: build edges from first differing chars of adjacent words,
// then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type ByteHeap []byte

func (h ByteHeap) Len() int            { return len(h) }
func (h ByteHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h ByteHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *ByteHeap) Push(x interface{}) { *h = append(*h, x.(byte)) }
func (h *ByteHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func alienOrder(words []string) []byte {
	adj := map[byte]map[byte]bool{}
	indeg := map[byte]int{}
	for _, w := range words {
		for i := 0; i < len(w); i++ {
			if adj[w[i]] == nil {
				adj[w[i]] = map[byte]bool{}
			}
			if _, ok := indeg[w[i]]; !ok {
				indeg[w[i]] = 0
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := words[i], words[i+1]
		n := len(a)
		if len(b) < n {
			n = len(b)
		}
		j := 0
		for ; j < n; j++ {
			if a[j] != b[j] {
				if !adj[a[j]][b[j]] {
					adj[a[j]][b[j]] = true
					indeg[b[j]]++
				}
				break
			}
		}
		if j == n && len(a) > len(b) {
			return nil // invalid prefix
		}
	}
	h := &ByteHeap{}
	heap.Init(h)
	for c, d := range indeg {
		if d == 0 {
			heap.Push(h, c)
		}
	}
	var order []byte
	for h.Len() > 0 {
		c := heap.Pop(h).(byte)
		order = append(order, c)
		nxts := []byte{}
		for nx := range adj[c] {
			nxts = append(nxts, nx)
		}
		for _, nx := range nxts {
			indeg[nx]--
			if indeg[nx] == 0 {
				heap.Push(h, nx)
			}
		}
	}
	if len(order) != len(indeg) {
		return nil
	}
	return order
}

func main() {
	words := []string{"xww", "wxyz", "wxyw", "ywx", "ywz"}
	order := alienOrder(words)
	parts := make([]string, len(order))
	for i, c := range order {
		parts[i] = "'" + string(c) + "'"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
