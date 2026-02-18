// Day 1096: LFU Cache with O(1) get/set.
// Approach: key->node maps + freq->doubly linked list (LRU order) + minFreq.
// Time: O(1) per op. Space: O(n).
package main

import (
	"container/list"
	"fmt"
)

type entry struct {
	key, value, freq int
}

type LFUCache struct {
	cap     int
	minFreq int
	nodes   map[int]*list.Element // key -> element
	lists   map[int]*list.List    // freq -> list (front = most recent)
}

func NewLFU(n int) *LFUCache {
	return &LFUCache{cap: n, nodes: map[int]*list.Element{}, lists: map[int]*list.List{}}
}

func (c *LFUCache) touch(el *list.Element) {
	e := el.Value.(*entry)
	c.lists[e.freq].Remove(el)
	if c.lists[e.freq].Len() == 0 {
		delete(c.lists, e.freq)
		if c.minFreq == e.freq {
			c.minFreq++
		}
	}
	e.freq++
	if c.lists[e.freq] == nil {
		c.lists[e.freq] = list.New()
	}
	c.nodes[e.key] = c.lists[e.freq].PushFront(e)
}

// Get returns value, ok.
func (c *LFUCache) Get(key int) (int, bool) {
	el, ok := c.nodes[key]
	if !ok {
		return 0, false
	}
	c.touch(el)
	return el.Value.(*entry).value, true
}

func (c *LFUCache) Set(key, value int) {
	if c.cap <= 0 {
		return
	}
	if el, ok := c.nodes[key]; ok {
		el.Value.(*entry).value = value
		c.touch(el)
		return
	}
	if len(c.nodes) >= c.cap {
		back := c.lists[c.minFreq].Back()
		ev := back.Value.(*entry)
		c.lists[c.minFreq].Remove(back)
		if c.lists[c.minFreq].Len() == 0 {
			delete(c.lists, c.minFreq)
		}
		delete(c.nodes, ev.key)
	}
	e := &entry{key, value, 1}
	if c.lists[1] == nil {
		c.lists[1] = list.New()
	}
	c.nodes[key] = c.lists[1].PushFront(e)
	c.minFreq = 1
}

func main() {
	c := NewLFU(2)
	c.Set(1, 1)
	c.Set(2, 2)
	if v, ok := c.Get(1); ok {
		fmt.Println(v) // 1
	}
	c.Set(3, 3) // evicts key 2
	if _, ok := c.Get(2); !ok {
		fmt.Println("null") // null
	}
	if v, ok := c.Get(3); ok {
		fmt.Println(v) // 3
	}
}
