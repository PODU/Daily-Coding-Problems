// LFU cache: key->(value,freq), freq->doubly linked list of keys (LRU within freq), track minFreq. O(1) get/set.
package main

import (
	"container/list"
	"fmt"
)

type entry struct {
	value int
	freq  int
	node  *list.Element // position in freqList[freq]
}

type LFUCache struct {
	cap      int
	minFreq  int
	kv       map[int]*entry
	freqList map[int]*list.List // freq -> list of keys (front = most recent)
}

func NewLFU(capacity int) *LFUCache {
	return &LFUCache{
		cap:      capacity,
		kv:       make(map[int]*entry),
		freqList: make(map[int]*list.List),
	}
}

func (c *LFUCache) touch(key int) {
	e := c.kv[key]
	f := e.freq
	c.freqList[f].Remove(e.node)
	if c.freqList[f].Len() == 0 {
		delete(c.freqList, f)
		if c.minFreq == f {
			c.minFreq = f + 1
		}
	}
	nf := f + 1
	e.freq = nf
	if c.freqList[nf] == nil {
		c.freqList[nf] = list.New()
	}
	e.node = c.freqList[nf].PushFront(key)
}

func (c *LFUCache) Get(key int) (int, bool) {
	e, ok := c.kv[key]
	if !ok {
		return 0, false
	}
	c.touch(key)
	return e.value, true
}

func (c *LFUCache) Set(key, value int) {
	if c.cap <= 0 {
		return
	}
	if e, ok := c.kv[key]; ok {
		e.value = value
		c.touch(key)
		return
	}
	if len(c.kv) >= c.cap {
		l := c.freqList[c.minFreq]
		back := l.Back() // LRU at min freq
		evict := back.Value.(int)
		l.Remove(back)
		if l.Len() == 0 {
			delete(c.freqList, c.minFreq)
		}
		delete(c.kv, evict)
	}
	if c.freqList[1] == nil {
		c.freqList[1] = list.New()
	}
	node := c.freqList[1].PushFront(key)
	c.kv[key] = &entry{value: value, freq: 1, node: node}
	c.minFreq = 1
}

func main() {
	c := NewLFU(2)
	c.Set(1, 1)
	c.Set(2, 2)
	show(c.Get(1)) // 1
	c.Set(3, 3)    // evicts key 2
	show(c.Get(2)) // null
	show(c.Get(3)) // 3
	c.Set(4, 4)    // evicts key 1
	show(c.Get(1)) // null
	show(c.Get(3)) // 3
	show(c.Get(4)) // 4
}

func show(v int, ok bool) {
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(v)
	}
}
