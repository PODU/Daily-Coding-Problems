// Day 575: 2D iterator over an array of arrays without flattening/cloning.
// Track (row, col); advance over empty rows. Next/HasNext amortized O(1).
package main

import (
	"fmt"
	"strings"
)

type Iterator2D struct {
	data     [][]int
	row, col int
}

func NewIterator2D(data [][]int) *Iterator2D {
	it := &Iterator2D{data: data}
	it.skipEmpty()
	return it
}

func (it *Iterator2D) skipEmpty() {
	for it.row < len(it.data) && it.col >= len(it.data[it.row]) {
		it.row++
		it.col = 0
	}
}

func (it *Iterator2D) HasNext() bool {
	return it.row < len(it.data)
}

func (it *Iterator2D) Next() int {
	if !it.HasNext() {
		panic("no more elements")
	}
	v := it.data[it.row][it.col]
	it.col++
	it.skipEmpty()
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
