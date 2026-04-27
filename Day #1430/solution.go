// Day 1430: Space-efficient SparseArray for a mostly-zero array.
// Approach: store only non-zero indices in a map. Time: O(1) avg per op, Space: O(#nonzero).
package main

import "fmt"

type SparseArray struct {
	data map[int]int
	n    int
}

func (s *SparseArray) Init(arr []int, size int) {
	s.n = size
	s.data = make(map[int]int)
	for i := 0; i < size; i++ {
		if arr[i] != 0 {
			s.data[i] = arr[i]
		}
	}
}

func (s *SparseArray) Set(i, val int) {
	if i < 0 || i >= s.n {
		panic("index out of bounds")
	}
	if val == 0 {
		delete(s.data, i)
	} else {
		s.data[i] = val
	}
}

func (s *SparseArray) Get(i int) int {
	if i < 0 || i >= s.n {
		panic("index out of bounds")
	}
	return s.data[i]
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
