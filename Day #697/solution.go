// Day 697: LRU cache with O(1) get/set.
// Approach: hash map (key -> list element) + container/list doubly linked list.
// get/set O(1) time, O(n) space.
package main

import (
	"container/list"
	"fmt"
)

type entry struct{ key, val int }

type LRUCache struct {
	cap int
	ll  *list.List
	m   map[int]*list.Element
}

func newLRU(n int) *LRUCache {
	return &LRUCache{cap: n, ll: list.New(), m: make(map[int]*list.Element)}
}

func (c *LRUCache) get(key int) (int, bool) {
	if el, ok := c.m[key]; ok {
		c.ll.MoveToFront(el)
		return el.Value.(*entry).val, true
	}
	return 0, false
}

func (c *LRUCache) set(key, value int) {
	if el, ok := c.m[key]; ok {
		el.Value.(*entry).val = value
		c.ll.MoveToFront(el)
		return
	}
	if c.ll.Len() == c.cap {
		back := c.ll.Back()
		c.ll.Remove(back)
		delete(c.m, back.Value.(*entry).key)
	}
	c.m[key] = c.ll.PushFront(&entry{key, value})
}

func main() {
	c := newLRU(2)
	c.set(1, 1)
	c.set(2, 2)
	v, _ := c.get(1)
	fmt.Println(v) // 1
	c.set(3, 3)    // evicts key 2
	if _, ok := c.get(2); !ok {
		fmt.Println("null")
	}
	v, _ = c.get(3)
	fmt.Println(v) // 3
}
