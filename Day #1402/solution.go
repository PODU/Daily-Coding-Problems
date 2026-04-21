// Huffman coding: merge the two lowest-frequency nodes via a min-heap to build
// an optimal prefix tree, then DFS to assign codes (left=0, right=1).
// Time O(C log C) for C distinct chars, Space O(C).
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
	x := old[n-1]
	*p = old[:n-1]
	return x
}

func dfs(n *Node, path string, codes map[byte]string) {
	if n.l == nil && n.r == nil {
		if path == "" {
			path = "0"
		}
		codes[n.ch] = path
		return
	}
	dfs(n.l, path+"0", codes)
	dfs(n.r, path+"1", codes)
}

func main() {
	chars := []byte{'c', 'a', 't', 's'}
	freqs := []int{1, 4, 3, 2}
	pq := &PQ{}
	heap.Init(pq)
	cnt := 0
	for i, c := range chars {
		heap.Push(pq, &Node{freq: freqs[i], ch: c, order: cnt})
		cnt++
	}
	for pq.Len() > 1 {
		a := heap.Pop(pq).(*Node)
		b := heap.Pop(pq).(*Node)
		heap.Push(pq, &Node{freq: a.freq + b.freq, order: cnt, l: a, r: b})
		cnt++
	}
	root := heap.Pop(pq).(*Node)
	codes := map[byte]string{}
	dfs(root, "", codes)
	keys := []byte{}
	for k := range codes {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	for _, k := range keys {
		fmt.Printf("%c: %s\n", k, codes[k])
	}
	enc := ""
	for _, c := range []byte("cats") {
		enc += codes[c]
	}
	fmt.Println("encode(cats):", enc)
}
