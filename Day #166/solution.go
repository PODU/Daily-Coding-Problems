// 2D iterator: track row/col indices, advance() skips empty subarrays. O(1) amortized per Next/HasNext, O(1) extra space.
package main

import (
	"fmt"
	"strings"
)

type Iterator2D struct {
	data     [][]int
	row, col int
}

func NewIterator2D(d [][]int) *Iterator2D {
	it := &Iterator2D{data: d}
	it.advance()
	return it
}

func (it *Iterator2D) advance() {
	for it.row < len(it.data) && it.col >= len(it.data[it.row]) {
		it.row++
		it.col = 0
	}
}

func (it *Iterator2D) HasNext() bool {
	it.advance()
	return it.row < len(it.data)
}

func (it *Iterator2D) Next() int {
	if !it.HasNext() {
		panic("no more elements")
	}
	v := it.data[it.row][it.col]
	it.col++
	return v
}

func main() {
	arr := [][]int{{1, 2}, {3}, {}, {4, 5, 6}}
	it := NewIterator2D(arr)
	var out []string
	for it.HasNext() {
		out = append(out, fmt.Sprintf("%d", it.Next()))
	}
	fmt.Println(strings.Join(out, ", "))
}
