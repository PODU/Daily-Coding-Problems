// Day 848: LRU cache with O(1) get/set using a hash map + container/list doubly linked list.
// Front = MRU, evict from back. O(1) per op.
package main

import (
	"container/list"
	"fmt"
)

type entry struct {
	key, value int
}

type LRUCache struct {
	cap int
	ll  *list.List
	m   map[int]*list.Element
}

func NewLRU(n int) *LRUCache {
	return &LRUCache{cap: n, ll: list.New(), m: make(map[int]*list.Element)}
}

// returns (value, ok); ok=false means null
func (c *LRUCache) Get(key int) (int, bool) {
	if el, ok := c.m[key]; ok {
		c.ll.MoveToFront(el)
		return el.Value.(*entry).value, true
	}
	return 0, false
}

func (c *LRUCache) Set(key, value int) {
	if el, ok := c.m[key]; ok {
		el.Value.(*entry).value = value
		c.ll.MoveToFront(el)
		return
	}
	if c.ll.Len() == c.cap {
		back := c.ll.Back()
		if back != nil {
			c.ll.Remove(back)
			delete(c.m, back.Value.(*entry).key)
		}
	}
	c.m[key] = c.ll.PushFront(&entry{key, value})
}

func show(v int, ok bool) string {
	if !ok {
		return "null"
	}
	return fmt.Sprintf("%d", v)
}

func main() {
	c := NewLRU(2)
	c.Set(1, 1)
	c.Set(2, 2)
	v, ok := c.Get(1)
	fmt.Println(show(v, ok)) // 1
	c.Set(3, 3)              // evicts 2
	v, ok = c.Get(2)
	fmt.Println(show(v, ok)) // null
	c.Set(4, 4)              // evicts 1
	v, ok = c.Get(1)
	fmt.Println(show(v, ok)) // null
	v, ok = c.Get(3)
	fmt.Println(show(v, ok)) // 3
	v, ok = c.Get(4)
	fmt.Println(show(v, ok)) // 4
}
