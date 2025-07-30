// Day 52: LRU cache with hashmap + container/list doubly linked list. O(1) get/set.
// Time: O(1) per op, Space: O(n).
package main

import (
	"container/list"
	"fmt"
)

type entry struct {
	key, value int
}

type LRUCache struct {
	cap   int
	ll    *list.List
	items map[int]*list.Element
}

func NewLRUCache(n int) *LRUCache {
	return &LRUCache{cap: n, ll: list.New(), items: make(map[int]*list.Element)}
}

// get returns value and ok=false when missing (ok=false denotes null).
func (c *LRUCache) get(key int) (int, bool) {
	if el, ok := c.items[key]; ok {
		c.ll.MoveToFront(el)
		return el.Value.(*entry).value, true
	}
	return 0, false
}

func (c *LRUCache) set(key, value int) {
	if el, ok := c.items[key]; ok {
		el.Value.(*entry).value = value
		c.ll.MoveToFront(el)
		return
	}
	if c.ll.Len() == c.cap {
		back := c.ll.Back()
		if back != nil {
			c.ll.Remove(back)
			delete(c.items, back.Value.(*entry).key)
		}
	}
	c.items[key] = c.ll.PushFront(&entry{key, value})
}

func show(v int, ok bool) string {
	if !ok {
		return "null"
	}
	return fmt.Sprintf("%d", v)
}

func main() {
	c := NewLRUCache(2)
	c.set(1, 1)
	c.set(2, 2)
	v, ok := c.get(1)
	fmt.Println(show(v, ok)) // 1
	c.set(3, 3)              // evicts key 2
	v, ok = c.get(2)
	fmt.Println(show(v, ok)) // null
	c.set(4, 4)              // evicts key 1
	v, ok = c.get(1)
	fmt.Println(show(v, ok)) // null
	v, ok = c.get(3)
	fmt.Println(show(v, ok)) // 3
	v, ok = c.get(4)
	fmt.Println(show(v, ok)) // 4
}
