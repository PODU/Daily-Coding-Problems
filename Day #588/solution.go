// Day 588: Space-efficient SparseArray backed by a map of non-zero entries.
// Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
package main

import "fmt"

type SparseArray struct {
	data map[int]int
	size int
}

func (s *SparseArray) Init(arr []int, size int) {
	s.size = size
	s.data = make(map[int]int)
	for i := 0; i < len(arr) && i < size; i++ {
		if arr[i] != 0 {
			s.data[i] = arr[i]
		}
	}
}

func (s *SparseArray) Set(i, val int) {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	if val == 0 {
		delete(s.data, i)
	} else {
		s.data[i] = val
	}
}

func (s *SparseArray) Get(i int) int {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	return s.data[i]
}

func main() {
	var sa SparseArray
	sa.Init([]int{0, 0, 0, 5, 0, 0, 9, 0}, 8)
	fmt.Println("get(3) =", sa.Get(3)) // 5
	fmt.Println("get(6) =", sa.Get(6)) // 9
	fmt.Println("get(0) =", sa.Get(0)) // 0
	sa.Set(1, 42)
	fmt.Println("after set(1,42), get(1) =", sa.Get(1)) // 42
	sa.Set(3, 0)
	fmt.Println("after set(3,0), get(3) =", sa.Get(3)) // 0
}
