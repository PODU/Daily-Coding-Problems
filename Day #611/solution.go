// Day 611: All O(1) structure (plus / minus / get_max / get_min).
// Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map. All ops O(1).
package main

import "fmt"

type Bucket struct {
	value      int
	keys       map[string]struct{}
	prev, next *Bucket
}

func newBucket(v int) *Bucket {
	return &Bucket{value: v, keys: make(map[string]struct{})}
}

type AllOne struct {
	head, tail *Bucket
	keyTo      map[string]*Bucket
}

func NewAllOne() *AllOne {
	h, t := newBucket(0), newBucket(0)
	h.next, t.prev = t, h
	return &AllOne{head: h, tail: t, keyTo: make(map[string]*Bucket)}
}

func (a *AllOne) insertAfter(node *Bucket, value int) *Bucket {
	b := newBucket(value)
	b.prev, b.next = node, node.next
	node.next.prev = b
	node.next = b
	return b
}

func (a *AllOne) remove(node *Bucket) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

func anyKey(b *Bucket) string {
	best := ""
	for k := range b.keys {
		if best == "" || k < best {
			best = k
		}
	}
	return best
}

func (a *AllOne) Plus(key string) {
	if cur, ok := a.keyTo[key]; ok {
		nxt := cur.next
		if nxt == a.tail || nxt.value != cur.value+1 {
			nxt = a.insertAfter(cur, cur.value+1)
		}
		nxt.keys[key] = struct{}{}
		a.keyTo[key] = nxt
		delete(cur.keys, key)
		if len(cur.keys) == 0 {
			a.remove(cur)
		}
	} else {
		first := a.head.next
		if first == a.tail || first.value != 1 {
			first = a.insertAfter(a.head, 1)
		}
		first.keys[key] = struct{}{}
		a.keyTo[key] = first
	}
}

func (a *AllOne) Minus(key string) {
	cur, ok := a.keyTo[key]
	if !ok {
		return
	}
	if cur.value == 1 {
		delete(a.keyTo, key)
	} else {
		prv := cur.prev
		if prv == a.head || prv.value != cur.value-1 {
			prv = a.insertAfter(cur.prev, cur.value-1)
		}
		prv.keys[key] = struct{}{}
		a.keyTo[key] = prv
	}
	delete(cur.keys, key)
	if len(cur.keys) == 0 {
		a.remove(cur)
	}
}

func (a *AllOne) GetMax() string {
	if a.tail.prev == a.head {
		return ""
	}
	return anyKey(a.tail.prev)
}

func (a *AllOne) GetMin() string {
	if a.head.next == a.tail {
		return ""
	}
	return anyKey(a.head.next)
}

func main() {
	a := NewAllOne()
	a.Plus("a")
	a.Plus("b")
	a.Plus("a") // a=2, b=1
	fmt.Println(a.GetMax()) // a
	fmt.Println(a.GetMin()) // b
}
