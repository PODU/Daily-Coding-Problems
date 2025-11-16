// Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
// Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).
package main

import "fmt"

type Node struct {
	sum int64
	ch  map[rune]*Node
}

func newNode() *Node { return &Node{ch: make(map[rune]*Node)} }

type PrefixMapSum struct {
	root   *Node
	values map[string]int64
}

func NewPrefixMapSum() *PrefixMapSum {
	return &PrefixMapSum{root: newNode(), values: make(map[string]int64)}
}

func (p *PrefixMapSum) Insert(key string, value int64) {
	delta := value - p.values[key]
	p.values[key] = value
	node := p.root
	for _, c := range key {
		if node.ch[c] == nil {
			node.ch[c] = newNode()
		}
		node = node.ch[c]
		node.sum += delta
	}
}

func (p *PrefixMapSum) Sum(prefix string) int64 {
	node := p.root
	for _, c := range prefix {
		if node.ch[c] == nil {
			return 0
		}
		node = node.ch[c]
	}
	return node.sum
}

func main() {
	m := NewPrefixMapSum()
	m.Insert("columnar", 3)
	fmt.Println(m.Sum("col")) // 3
	m.Insert("column", 2)
	fmt.Println(m.Sum("col")) // 5
}
