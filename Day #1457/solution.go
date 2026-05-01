// LFU cache: map key->node, map freq->doubly-linked-list, track minFreq. O(1) per op.
// Time: O(1) get/set. Space: O(capacity).
package main

import (
	"fmt"
	"strconv"
)

type node struct {
	key, val, freq int
	prev, next     *node
}

type dll struct {
	head, tail *node
	size       int
}

func newDLL() *dll {
	h := &node{}
	t := &node{}
	h.next = t
	t.prev = h
	return &dll{head: h, tail: t}
}

func (d *dll) addFront(n *node) {
	n.prev = d.head
	n.next = d.head.next
	d.head.next.prev = n
	d.head.next = n
	d.size++
}

func (d *dll) remove(n *node) {
	n.prev.next = n.next
	n.next.prev = n.prev
	d.size--
}

func (d *dll) removeLast() *node {
	if d.size == 0 {
		return nil
	}
	n := d.tail.prev
	d.remove(n)
	return n
}

type LFUCache struct {
	cap     int
	minFreq int
	nodes   map[int]*node
	freqs   map[int]*dll
}

func NewLFUCache(capacity int) *LFUCache {
	return &LFUCache{
		cap:   capacity,
		nodes: make(map[int]*node),
		freqs: make(map[int]*dll),
	}
}

func (c *LFUCache) touch(n *node) {
	f := n.freq
	d := c.freqs[f]
	d.remove(n)
	if d.size == 0 {
		delete(c.freqs, f)
		if c.minFreq == f {
			c.minFreq++
		}
	}
	n.freq++
	if c.freqs[n.freq] == nil {
		c.freqs[n.freq] = newDLL()
	}
	c.freqs[n.freq].addFront(n)
}

// Get returns (value, true) or (0, false) if absent.
func (c *LFUCache) Get(key int) (int, bool) {
	n, ok := c.nodes[key]
	if !ok {
		return 0, false
	}
	c.touch(n)
	return n.val, true
}

func (c *LFUCache) Set(key, value int) {
	if c.cap <= 0 {
		return
	}
	if n, ok := c.nodes[key]; ok {
		n.val = value
		c.touch(n)
		return
	}
	if len(c.nodes) >= c.cap {
		lru := c.freqs[c.minFreq].removeLast()
		delete(c.nodes, lru.key)
	}
	n := &node{key: key, val: value, freq: 1}
	c.nodes[key] = n
	if c.freqs[1] == nil {
		c.freqs[1] = newDLL()
	}
	c.freqs[1].addFront(n)
	c.minFreq = 1
}

func fmtVal(v int, ok bool) string {
	if !ok {
		return "null"
	}
	return strconv.Itoa(v)
}

func main() {
	cache := NewLFUCache(2)
	cache.Set(1, 1)
	cache.Set(2, 2)
	v, ok := cache.Get(1)
	fmt.Println(fmtVal(v, ok)) // 1
	cache.Set(3, 3)            // evicts key 2
	v, ok = cache.Get(2)
	fmt.Println(fmtVal(v, ok)) // null
	cache.Get(3)               // access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
	cache.Set(4, 4)            // evicts key 1
	v, ok = cache.Get(1)
	fmt.Println(fmtVal(v, ok)) // null
	v, ok = cache.Get(3)
	fmt.Println(fmtVal(v, ok)) // 3
	v, ok = cache.Get(4)
	fmt.Println(fmtVal(v, ok)) // 4
}
