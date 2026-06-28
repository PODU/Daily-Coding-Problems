// LRU cache via map + container/list doubly linked list. O(1) per get/set. Space O(capacity).
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

// get returns (value, true) if present, else (0, false).
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
	el := c.ll.PushFront(&entry{key, value})
	c.items[key] = el
}

func printGet(c *LRUCache, key int) {
	if v, ok := c.get(key); ok {
		fmt.Println(v)
	} else {
		fmt.Println("null")
	}
}

func main() {
	c := NewLRUCache(2)
	c.set(1, 1)
	c.set(2, 2)
	printGet(c, 1) // 1
	c.set(3, 3)    // evicts 2
	printGet(c, 2) // null
	c.set(4, 4)    // evicts 1
	printGet(c, 1) // null
	printGet(c, 3) // 3
	printGet(c, 4) // 4
}
