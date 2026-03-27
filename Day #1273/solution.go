// Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
// Trie storing accumulated sums; insert applies the delta vs the old value.
// insert/sum are O(key length).
package main

import "fmt"

type trieNode struct {
	sum  int
	next map[rune]*trieNode
}

func newTrieNode() *trieNode { return &trieNode{next: map[rune]*trieNode{}} }

type PrefixMapSum struct {
	root *trieNode
	vals map[string]int
}

func NewPrefixMapSum() *PrefixMapSum {
	return &PrefixMapSum{root: newTrieNode(), vals: map[string]int{}}
}

func (p *PrefixMapSum) Insert(key string, value int) {
	delta := value - p.vals[key]
	p.vals[key] = value
	node := p.root
	for _, c := range key {
		if node.next[c] == nil {
			node.next[c] = newTrieNode()
		}
		node = node.next[c]
		node.sum += delta
	}
}

func (p *PrefixMapSum) Sum(prefix string) int {
	node := p.root
	for _, c := range prefix {
		if node.next[c] == nil {
			return 0
		}
		node = node.next[c]
	}
	return node.sum
}

func main() {
	mapsum := NewPrefixMapSum()
	mapsum.Insert("columnar", 3)
	fmt.Println(mapsum.Sum("col")) // 3
	mapsum.Insert("column", 2)
	fmt.Println(mapsum.Sum("col")) // 5
}
