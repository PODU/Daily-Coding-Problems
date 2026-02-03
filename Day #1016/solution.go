// Huffman coding: min-heap repeatedly merges two smallest nodes, then DFS assigns codes (left='0', right='1').
// Tie-break by insertion order for determinism. O(k log k) time, O(k) space.
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type Node struct {
	freq, order int
	ch          byte
	l, r        *Node
}

type PQ []*Node

func (p PQ) Len() int { return len(p) }
func (p PQ) Less(i, j int) bool {
	if p[i].freq != p[j].freq {
		return p[i].freq < p[j].freq
	}
	return p[i].order < p[j].order
}
func (p PQ) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }
func (p *PQ) Push(x interface{}) { *p = append(*p, x.(*Node)) }
func (p *PQ) Pop() interface{} {
	old := *p
	n := len(old)
	v := old[n-1]
	*p = old[:n-1]
	return v
}

func huffman(freqs map[byte]int) map[byte]string {
	var keys []byte
	for k := range freqs {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })

	pq := &PQ{}
	heap.Init(pq)
	order := 0
	for _, k := range keys {
		heap.Push(pq, &Node{freqs[k], order, k, nil, nil})
		order++
	}
	for pq.Len() > 1 {
		a := heap.Pop(pq).(*Node)
		b := heap.Pop(pq).(*Node)
		heap.Push(pq, &Node{a.freq + b.freq, order, 0, a, b})
		order++
	}
	root := heap.Pop(pq).(*Node)

	codes := map[byte]string{}
	var dfs func(n *Node, code string)
	dfs = func(n *Node, code string) {
		if n.l == nil && n.r == nil {
			if code == "" {
				code = "0"
			}
			codes[n.ch] = code
			return
		}
		dfs(n.l, code+"0")
		dfs(n.r, code+"1")
	}
	dfs(root, "")
	return codes
}

func main() {
	freqs := map[byte]int{'a': 5, 'b': 9, 'c': 12, 'd': 13, 'e': 16, 'f': 45}
	codes := huffman(freqs)
	var keys []byte
	for k := range codes {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	for _, k := range keys {
		fmt.Printf("%c: %s\n", k, codes[k])
	}
}
