// All O(1) data structure (LeetCode 432).
// Doubly linked list of count-buckets (each holds a set of keys) + map key->bucket. All ops O(1); space O(N).
package main

import "fmt"

type Bucket struct {
	count      int
	keys       map[string]struct{}
	prev, next *Bucket
}

func newBucket(count int) *Bucket {
	return &Bucket{count: count, keys: make(map[string]struct{})}
}

type AllOne struct {
	head, tail *Bucket
	keyBucket  map[string]*Bucket
}

func NewAllOne() *AllOne {
	h := newBucket(-1 << 62)
	t := newBucket(1 << 62)
	h.next = t
	t.prev = h
	return &AllOne{head: h, tail: t, keyBucket: make(map[string]*Bucket)}
}

func (a *AllOne) insertAfter(node *Bucket, count int) *Bucket {
	b := newBucket(count)
	b.prev, b.next = node, node.next
	node.next.prev = b
	node.next = b
	return b
}

func (a *AllOne) remove(b *Bucket) {
	b.prev.next = b.next
	b.next.prev = b.prev
}

func (a *AllOne) Plus(key string) {
	if cur, ok := a.keyBucket[key]; ok {
		nxt := cur.next
		if nxt == a.tail || nxt.count != cur.count+1 {
			nxt = a.insertAfter(cur, cur.count+1)
		}
		nxt.keys[key] = struct{}{}
		a.keyBucket[key] = nxt
		delete(cur.keys, key)
		if len(cur.keys) == 0 {
			a.remove(cur)
		}
	} else {
		first := a.head.next
		if first == a.tail || first.count != 1 {
			first = a.insertAfter(a.head, 1)
		}
		first.keys[key] = struct{}{}
		a.keyBucket[key] = first
	}
}

func (a *AllOne) Minus(key string) {
	cur, ok := a.keyBucket[key]
	if !ok {
		return
	}
	if cur.count == 1 {
		delete(a.keyBucket, key)
	} else {
		prv := cur.prev
		if prv == a.head || prv.count != cur.count-1 {
			prv = a.insertAfter(cur.prev, cur.count-1)
		}
		prv.keys[key] = struct{}{}
		a.keyBucket[key] = prv
	}
	delete(cur.keys, key)
	if len(cur.keys) == 0 {
		a.remove(cur)
	}
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
	a.Plus("a")
	a.Plus("a")
	a.Plus("b")
	fmt.Println("get_max:", a.GetMax())
	fmt.Println("get_min:", a.GetMin())
	a.Minus("a")
	a.Minus("a")
	fmt.Println("get_max:", a.GetMax())
	fmt.Println("get_min:", a.GetMin())
}
