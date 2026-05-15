// Huffman coding: build tree via min-heap, merge two smallest nodes, assign 0/1 edges.
// Time: O(n log n) for n symbols. Space: O(n).
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type Node struct {
	ch    byte
	freq  int64
	order int
	l, r  *Node
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

func build(n *Node, code string, out map[byte]string) {
	if n == nil {
		return
	}
	if n.l == nil && n.r == nil {
		if code == "" {
			code = "0"
		}
		out[n.ch] = code
		return
	}
	build(n.l, code+"0", out)
	build(n.r, code+"1", out)
}

func main() {
	chars := []byte{'a', 'b', 'c', 'd', 'e', 'f'}
	freqs := []int64{5, 9, 12, 13, 16, 45}
	fmap := map[byte]int64{}
	pq := &PQ{}
	heap.Init(pq)
	counter := 0
	for i, c := range chars {
		heap.Push(pq, &Node{ch: c, freq: freqs[i], order: counter})
		fmap[c] = freqs[i]
		counter++
	}
	for pq.Len() > 1 {
		a := heap.Pop(pq).(*Node)
		b := heap.Pop(pq).(*Node)
		m := &Node{ch: 0, freq: a.freq + b.freq, order: counter, l: a, r: b}
		counter++
		heap.Push(pq, m)
	}
	root := heap.Pop(pq).(*Node)
	codes := map[byte]string{}
	build(root, "", codes)

	keys := make([]byte, 0, len(codes))
	for k := range codes {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
	var totalBits int64
	for _, k := range keys {
		fmt.Printf("%c: %s\n", k, codes[k])
		totalBits += int64(len(codes[k])) * fmap[k]
	}
	fmt.Printf("Total encoded bits: %d\n", totalBits)
}
