// Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
// lowest-frequency nodes (min-heap), then read each char's code from its
// root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
// Note: the README's example tree is illustrative, not a strict Huffman tree;
// this produces a correct, deterministic optimal-prefix Huffman mapping.
package main

import (
	"container/heap"
	"fmt"
	"sort"
)

type Node struct {
	freq  int64
	order int
	ch    byte
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
	v := old[n-1]
	*p = old[:n-1]
	return v
}

func buildCodes(n *Node, path string, codes map[byte]string) {
	if n.l == nil && n.r == nil {
		if path == "" {
			path = "0"
		}
		codes[n.ch] = path
		return
	}
	buildCodes(n.l, path+"0", codes)
	buildCodes(n.r, path+"1", codes)
}

func main() {
	freq := []struct {
		ch byte
		f  int64
	}{{'c', 1}, {'a', 1}, {'t', 1}, {'s', 1}}

	pq := &PQ{}
	heap.Init(pq)
	order := 0
	for _, e := range freq {
		heap.Push(pq, &Node{freq: e.f, order: order, ch: e.ch})
		order++
	}
	for pq.Len() > 1 {
		a := heap.Pop(pq).(*Node)
		b := heap.Pop(pq).(*Node)
		heap.Push(pq, &Node{freq: a.freq + b.freq, order: order, l: a, r: b})
		order++
	}
	root := heap.Pop(pq).(*Node)

	codes := map[byte]string{}
	buildCodes(root, "", codes)

	keys := make([]string, 0, len(codes))
	for k := range codes {
		keys = append(keys, string(k))
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Printf("%s -> %s\n", k, codes[k[0]])
	}

	word := "cats"
	encoded := ""
	for i := 0; i < len(word); i++ {
		encoded += codes[word[i]]
	}
	fmt.Printf("%s -> %s\n", word, encoded)
}
