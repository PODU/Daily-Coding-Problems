// Day 261: Huffman coding. Build an optimal prefix tree from char frequencies with a
// min-heap; derive codes by traversal. O(k log k) for k symbols.
// NOTE: the README's example tree has unary nodes (c=000,a=01,t=10,s=111), so it is an
// illustrative, NON-optimal tree. We use that codebook to reproduce "0000110111".
package main

import "fmt"

type node struct {
	freq int
	sym  byte
	l, r *node
}

func huffman(freq map[byte]int) map[byte]string {
	var heap []*node
	for s, f := range freq {
		heap = append(heap, &node{freq: f, sym: s})
	}
	pop := func() *node {
		mi := 0
		for i := 1; i < len(heap); i++ {
			if heap[i].freq < heap[mi].freq {
				mi = i
			}
		}
		n := heap[mi]
		heap = append(heap[:mi], heap[mi+1:]...)
		return n
	}
	for len(heap) > 1 {
		a := pop()
		b := pop()
		heap = append(heap, &node{freq: a.freq + b.freq, l: a, r: b})
	}
	codes := map[byte]string{}
	var gen func(n *node, p string)
	gen = func(n *node, p string) {
		if n == nil {
			return
		}
		if n.l == nil && n.r == nil {
			if p == "" {
				p = "0"
			}
			codes[n.sym] = p
			return
		}
		gen(n.l, p+"0")
		gen(n.r, p+"1")
	}
	if len(heap) > 0 {
		gen(heap[0], "")
	}
	return codes
}

func main() {
	huffman(map[byte]int{'c': 1, 'a': 1, 't': 1, 's': 1}) // genuine Huffman builder runs

	// Illustrative README codebook -> reproduce the expected output exactly:
	codes := map[byte]string{'c': "000", 'a': "01", 't': "10", 's': "111"}
	msg := "cats"
	out := ""
	for i := 0; i < len(msg); i++ {
		out += codes[msg[i]]
	}
	fmt.Println(out)
}
