// 2D iterator with lazy outer/inner pointers (no flatten/clone).
// Next() & HasNext() amortized O(1), Space O(1) extra.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Iterator2D struct {
	data         [][]int
	outer, inner int
}

func (it *Iterator2D) HasNext() bool {
	for it.outer < len(it.data) && it.inner >= len(it.data[it.outer]) {
		it.outer++
		it.inner = 0
	}
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
	it := &Iterator2D{data: [][]int{{1, 2}, {3}, {}, {4, 5, 6}}}
	var out []string
	for it.HasNext() {
		out = append(out, strconv.Itoa(it.Next()))
	}
	fmt.Println(strings.Join(out, ", "))
}
