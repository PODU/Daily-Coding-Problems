// PrefixMapSum: Trie where each node stores the running sum of values passing through it.
// On overwrite, propagate delta = new - old. insert/sum both O(key length).
package main

import "fmt"

type node struct {
	next map[byte]*node
	sum  int
}

func newNode() *node { return &node{next: map[byte]*node{}} }

type PrefixMapSum struct {
	root *node
	vals map[string]int
}

func NewPrefixMapSum() *PrefixMapSum {
	return &PrefixMapSum{root: newNode(), vals: map[string]int{}}
}

func (p *PrefixMapSum) Insert(key string, value int) {
	delta := value - p.vals[key]
	p.vals[key] = value
	n := p.root
	for i := 0; i < len(key); i++ {
		if n.next[key[i]] == nil {
			n.next[key[i]] = newNode()
		}
		n = n.next[key[i]]
		n.sum += delta
	}
}

func (p *PrefixMapSum) Sum(prefix string) int {
	n := p.root
	for i := 0; i < len(prefix); i++ {
		n = n.next[prefix[i]]
		if n == nil {
			return 0
		}
	}
	return n.sum
}

func main() {
	mapsum := NewPrefixMapSum()
	mapsum.Insert("columnar", 3)
	fmt.Println(mapsum.Sum("col")) // 3
	mapsum.Insert("column", 2)
	fmt.Println(mapsum.Sum("col")) // 5
}
