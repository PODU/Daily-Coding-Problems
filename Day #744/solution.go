// O(1) LFU cache. Each freq has a container/list (front = most recent);
// map key->*list.Element gives O(1) removal. Eviction: least-frequent, then least-recent.
// Time: O(1) per get/set, Space: O(capacity).
package main

import (
	"container/list"
	"fmt"
)

type LFUCache struct {
	cap     int
	minFreq int
	val     map[int]int
	freq    map[int]int
	buckets map[int]*list.List
	pos     map[int]*list.Element
}

func NewLFU(capacity int) *LFUCache {
	return &LFUCache{
		cap:     capacity,
		val:     map[int]int{},
		freq:    map[int]int{},
		buckets: map[int]*list.List{},
		pos:     map[int]*list.Element{},
	}
}

func (c *LFUCache) bump(key int) {
	f := c.freq[key]
	c.buckets[f].Remove(c.pos[key])
	if c.buckets[f].Len() == 0 {
		delete(c.buckets, f)
		if c.minFreq == f {
			c.minFreq++
		}
	}
	c.freq[key] = f + 1
	if c.buckets[f+1] == nil {
		c.buckets[f+1] = list.New()
	}
	c.pos[key] = c.buckets[f+1].PushFront(key)
}

// Get returns (value, found).
func (c *LFUCache) Get(key int) (int, bool) {
	if _, ok := c.val[key]; !ok {
		return 0, false
	}
	c.bump(key)
	return c.val[key], true
}

func (c *LFUCache) Set(key, value int) {
	if c.cap <= 0 {
		return
	}
	if _, ok := c.val[key]; ok {
		c.val[key] = value
		c.bump(key)
		return
	}
	if len(c.val) >= c.cap {
		b := c.buckets[c.minFreq]
		e := b.Back()
		evict := e.Value.(int)
		b.Remove(e)
		delete(c.val, evict)
		delete(c.freq, evict)
		delete(c.pos, evict)
	}
	c.val[key] = value
	c.freq[key] = 1
	if c.buckets[1] == nil {
		c.buckets[1] = list.New()
	}
	c.pos[key] = c.buckets[1].PushFront(key)
	c.minFreq = 1
}

func printGet(c *LFUCache, key int) {
	if v, ok := c.Get(key); ok {
		fmt.Println(v)
	} else {
		fmt.Println("null")
	}
}

func main() {
	c := NewLFU(2)
	c.Set(1, 1)
	c.Set(2, 2)
	printGet(c, 1) // 1
	c.Set(3, 3)    // evicts key 2
	printGet(c, 2) // null
	printGet(c, 3) // 3
	c.Set(4, 4)    // evicts key 1
	printGet(c, 1) // null
	printGet(c, 3) // 3
	printGet(c, 4) // 4
}
