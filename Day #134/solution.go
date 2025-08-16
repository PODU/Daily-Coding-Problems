// Day 134: SparseArray storing only non-zero entries in a hash map.
// init O(n) once, set/get O(1) average. Space O(#nonzero).
package main

import "fmt"

type SparseArray struct {
	data map[int]int
	size int
}

func (s *SparseArray) init(arr []int, sz int) {
	s.size = sz
	s.data = make(map[int]int)
	for i := 0; i < len(arr) && i < sz; i++ {
		if arr[i] != 0 {
			s.data[i] = arr[i]
		}
	}
}

func (s *SparseArray) set(i, val int) {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	if val == 0 {
		delete(s.data, i)
	} else {
		s.data[i] = val
	}
}

func (s *SparseArray) get(i int) int {
	if i < 0 || i >= s.size {
		panic("index out of range")
	}
	return s.data[i] // missing key returns 0
}

func main() {
	sa := &SparseArray{}
	sa.init([]int{0, 0, 7, 0, 0, 0, 3, 0}, 8)
	fmt.Println(sa.get(2)) // 7
	fmt.Println(sa.get(0)) // 0
	sa.set(0, 5)
	fmt.Println(sa.get(0)) // 5
	sa.set(2, 0)
	fmt.Println(sa.get(2)) // 0
}
