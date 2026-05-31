// Day 1584: 2D iterator over array of arrays (no flatten/clone).
// Maintain (outer,inner) indices; skip advances past empty inner arrays. Time: O(1) amortized; Space: O(1).
package main

import (
	"fmt"
	"strings"
)

type Iterator2D struct {
	data         [][]int
	outer, inner int
}

func NewIterator2D(d [][]int) *Iterator2D {
	it := &Iterator2D{data: d}
	it.skip()
	return it
}

func (it *Iterator2D) skip() {
	for it.outer < len(it.data) && it.inner >= len(it.data[it.outer]) {
		it.outer++
		it.inner = 0
	}
}

func (it *Iterator2D) HasNext() bool {
	it.skip()
	return it.outer < len(it.data)
}

func (it *Iterator2D) Next() int {
	if !it.HasNext() {
		panic("no more elements")
	}
	v := it.data[it.outer][it.inner]
	it.inner++
	return v
}

func main() {
	it := NewIterator2D([][]int{{1, 2}, {3}, {}, {4, 5, 6}})
	var out []string
	for it.HasNext() {
		out = append(out, fmt.Sprintf("%d", it.Next()))
	}
	fmt.Println(strings.Join(out, ", ")) // 1, 2, 3, 4, 5, 6
}
