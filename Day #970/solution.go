// Day 970: Space-efficient SparseArray storing only non-zero entries.
// Approach: map of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).
package main

import "fmt"

type SparseArray struct {
	m    map[int]int
	size int
}

func (s *SparseArray) Init(arr []int, size int) {
	s.size = size
	s.m = make(map[int]int)
	for i := 0; i < size; i++ {
		if arr[i] != 0 {
			s.m[i] = arr[i]
		}
	}
}

func (s *SparseArray) Set(i, val int) {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	if val == 0 {
		delete(s.m, i)
	} else {
		s.m[i] = val
	}
}

func (s *SparseArray) Get(i int) int {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	return s.m[i]
}

func main() {
	var sa SparseArray
	sa.Init([]int{0, 0, 5, 0, 0, 0, 9, 0}, 8)
	fmt.Println(sa.Get(2)) // 5
	fmt.Println(sa.Get(3)) // 0
	sa.Set(3, 7)
	fmt.Println(sa.Get(3)) // 7
	sa.Set(2, 0)
	fmt.Println(sa.Get(2)) // 0
}
