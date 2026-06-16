// Day 1676: PrefixMapSum via prefix-sum map + delta on overwrite.
// insert/sum both O(key length). Space O(total chars).
package main

import "fmt"

type PrefixMapSum struct {
	total map[string]int
	vals  map[string]int
}

func NewPrefixMapSum() *PrefixMapSum {
	return &PrefixMapSum{total: map[string]int{}, vals: map[string]int{}}
}

func (p *PrefixMapSum) Insert(key string, value int) {
	delta := value - p.vals[key]
	p.vals[key] = value
	prefix := ""
	for _, ch := range key {
		prefix += string(ch)
		p.total[prefix] += delta
	}
}

func (p *PrefixMapSum) Sum(prefix string) int {
	return p.total[prefix]
}

func main() {
	m := NewPrefixMapSum()
	m.Insert("columnar", 3)
	fmt.Println(m.Sum("col")) // 3
	m.Insert("column", 2)
	fmt.Println(m.Sum("col")) // 5
}
