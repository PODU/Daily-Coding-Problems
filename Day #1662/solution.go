// O(1) LFU cache: key->node map, freq->list(ordered by recency), minFreq pointer.
// get/set O(1); Space O(capacity). Evict least-freq, tie -> least-recently-used.
package main

import (
	"container/list"
	"fmt"
)

type LFU struct {
	cap, minFreq int
	vals         map[int]int
	freqs        map[int]int
	lists        map[int]*list.List       // freq -> list (front=MRU, back=LRU)
	iters        map[int]*list.Element     // key -> element
}

func NewLFU(c int) *LFU {
	return &LFU{cap: c, vals: map[int]int{}, freqs: map[int]int{},
		lists: map[int]*list.List{}, iters: map[int]*list.Element{}}
}
func (l *LFU) touch(key int) {
	f := l.freqs[key]
	l.lists[f].Remove(l.iters[key])
	if l.lists[f].Len() == 0 {
		delete(l.lists, f)
		if l.minFreq == f {
			l.minFreq++
		}
	}
	nf := f + 1
	l.freqs[key] = nf
	if l.lists[nf] == nil {
		l.lists[nf] = list.New()
	}
	l.iters[key] = l.lists[nf].PushFront(key)
}
func (l *LFU) get(key int) (int, bool) {
	if _, ok := l.vals[key]; !ok {
		return 0, false
	}
	l.touch(key)
	return l.vals[key], true
}
func (l *LFU) set(key, val int) {
	if l.cap <= 0 {
		return
	}
	if _, ok := l.vals[key]; ok {
		l.vals[key] = val
		l.touch(key)
		return
	}
	if len(l.vals) >= l.cap {
		back := l.lists[l.minFreq].Back()
		lru := back.Value.(int)
		l.lists[l.minFreq].Remove(back)
		if l.lists[l.minFreq].Len() == 0 {
			delete(l.lists, l.minFreq)
		}
		delete(l.vals, lru)
		delete(l.freqs, lru)
		delete(l.iters, lru)
	}
	l.vals[key] = val
	l.freqs[key] = 1
	if l.lists[1] == nil {
		l.lists[1] = list.New()
	}
	l.iters[key] = l.lists[1].PushFront(key)
	l.minFreq = 1
}
func show(v int, ok bool) {
	if !ok {
		fmt.Println("null")
	} else {
		fmt.Println(v)
	}
}
func main() {
	c := NewLFU(2)
	c.set(1, 1)
	c.set(2, 2)
	show(c.get(1))
	c.set(3, 3)
	show(c.get(2))
	show(c.get(3))
	c.set(4, 4)
	show(c.get(1))
	show(c.get(3))
	show(c.get(4))
}
