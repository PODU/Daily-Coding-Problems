// Day 368: KV store with update/get/maxKey(val).
// kv maps key->value; byVal maps value->sorted slice of keys (binary insert), so
// maxKey is the last element. update O(n) worst (slice shift), maxKey O(1).
package main

import (
	"fmt"
	"sort"
)

type KVStore struct {
	kv    map[int]int
	byVal map[int][]int
}

func NewKVStore() *KVStore {
	return &KVStore{kv: map[int]int{}, byVal: map[int][]int{}}
}

func (s *KVStore) Update(key, val int) {
	if old, ok := s.kv[key]; ok {
		lst := s.byVal[old]
		i := sort.SearchInts(lst, key)
		s.byVal[old] = append(lst[:i], lst[i+1:]...)
		if len(s.byVal[old]) == 0 {
			delete(s.byVal, old)
		}
	}
	s.kv[key] = val
	lst := s.byVal[val]
	i := sort.SearchInts(lst, key)
	lst = append(lst, 0)
	copy(lst[i+1:], lst[i:])
	lst[i] = key
	s.byVal[val] = lst
}

func (s *KVStore) Get(key int) (int, bool) {
	v, ok := s.kv[key]
	return v, ok
}

func (s *KVStore) MaxKey(val int) (int, bool) {
	lst, ok := s.byVal[val]
	if !ok || len(lst) == 0 {
		return 0, false
	}
	return lst[len(lst)-1], true
}

func main() {
	kv := NewKVStore()
	kv.Update(1, 1)
	kv.Update(2, 1)
	v, _ := kv.MaxKey(1)
	fmt.Println(v) // 2
}
