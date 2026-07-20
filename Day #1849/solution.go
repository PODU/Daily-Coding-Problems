// Day 1849: SparseArray storing only non-zero entries in a map.
// init O(N) once, set/get O(1) average. Space O(non-zero count).
package main

import "fmt"

type SparseArray struct {
	size int
	data map[int]int
}

func NewSparseArray(arr []int, size int) *SparseArray {
	s := &SparseArray{size: size, data: map[int]int{}}
	for i, v := range arr {
		if v != 0 {
			s.data[i] = v
		}
	}
	return s
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
	return s.data[i] // missing key -> 0
}

func main() {
	sa := NewSparseArray([]int{0, 0, 5, 0, 0, 0, 9, 0}, 8)
	fmt.Println(sa.Get(2)) // 5
	fmt.Println(sa.Get(3)) // 0
	sa.Set(3, 7)
	fmt.Println(sa.Get(3)) // 7
	sa.Set(2, 0)
	fmt.Println(sa.Get(2)) // 0
}
