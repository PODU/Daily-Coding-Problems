// LRU cache via hash map + doubly linked list; get/set O(1), evict LRU on overflow. O(1) time, O(n) space.
package main

import "fmt"

type node struct {
	key, val   int
	prev, next *node
}

type LRUCache struct {
	cap        int
	m          map[int]*node
	head, tail *node // head = most recent, tail = least recent (sentinels)
}

func NewLRUCache(capacity int) *LRUCache {
	h := &node{}
	t := &node{}
	h.next = t
	t.prev = h
	return &LRUCache{cap: capacity, m: make(map[int]*node), head: h, tail: t}
}

func (c *LRUCache) remove(n *node) {
	n.prev.next = n.next
	n.next.prev = n.prev
}

func (c *LRUCache) addFront(n *node) {
	n.next = c.head.next
	n.prev = c.head
	c.head.next.prev = n
	c.head.next = n
}

// Get returns (value, true) or (0, false) if missing.
func (c *LRUCache) Get(key int) (int, bool) {
	n, ok := c.m[key]
	if !ok {
		return 0, false
	}
	c.remove(n)
	c.addFront(n)
	return n.val, true
}

func (c *LRUCache) Set(key, value int) {
	if n, ok := c.m[key]; ok {
		n.val = value
		c.remove(n)
		c.addFront(n)
		return
	}
	if len(c.m) == c.cap {
		lru := c.tail.prev
		c.remove(lru)
		delete(c.m, lru.key)
	}
	n := &node{key: key, val: value}
	c.addFront(n)
	c.m[key] = n
}

func fmtGet(c *LRUCache, key int) string {
	if v, ok := c.Get(key); ok {
		return fmt.Sprintf("%d", v)
	}
	return "null"
}

func main() {
	cache := NewLRUCache(2)
	cache.Set(1, 1)
	cache.Set(2, 2)
	fmt.Println(fmtGet(cache, 1)) // 1
	cache.Set(3, 3)               // evicts key 2
	fmt.Println(fmtGet(cache, 2)) // null
	cache.Set(4, 4)               // evicts key 1
	fmt.Println(fmtGet(cache, 1)) // null
	fmt.Println(fmtGet(cache, 3)) // 3
	fmt.Println(fmtGet(cache, 4)) // 4
}
