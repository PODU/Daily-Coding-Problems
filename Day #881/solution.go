// 2D iterator over array of arrays without flattening. Next/HasNext amortized O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Iterator2D struct {
	data     [][]int
	row, col int
}

func NewIterator2D(d [][]int) *Iterator2D { return &Iterator2D{data: d} }

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
	it := NewIterator2D([][]int{{1, 2}, {3}, {}, {4, 5, 6}})
	out := []string{}
	for it.HasNext() {
		out = append(out, strconv.Itoa(it.Next()))
	}
	fmt.Println(strings.Join(out, ", "))
}
