// Day 799: PrefixMapSum - trie where each node stores sum of values below it.
// insert overwrites via delta (new-old) propagated along the path.
// insert O(L), sum O(L). Space O(total chars).
package main

import "fmt"

type Node struct {
	total int
	child map[rune]*Node
}

func newNode() *Node { return &Node{child: map[rune]*Node{}} }

type PrefixMapSum struct {
	root *Node
	vals map[string]int
}

func NewPrefixMapSum() *PrefixMapSum {
	return &PrefixMapSum{root: newNode(), vals: map[string]int{}}
}

func (p *PrefixMapSum) Insert(key string, value int) {
	delta := value - p.vals[key]
	p.vals[key] = value
	cur := p.root
	cur.total += delta
	for _, c := range key {
		if cur.child[c] == nil {
			cur.child[c] = newNode()
		}
		cur = cur.child[c]
		cur.total += delta
	}
}

func (p *PrefixMapSum) Sum(prefix string) int {
	cur := p.root
	for _, c := range prefix {
		if cur.child[c] == nil {
			return 0
		}
		cur = cur.child[c]
	}
	return cur.total
}

func main() {
	m := NewPrefixMapSum()
	m.Insert("columnar", 3)
	fmt.Println(m.Sum("col")) // 3
	m.Insert("column", 2)
	fmt.Println(m.Sum("col")) // 5
}
