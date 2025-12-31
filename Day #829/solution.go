// Day 829: O(1) data structure with plus/minus/get_max/get_min.
// Doubly-linked list of count-buckets (each a set of keys) + key->bucket map.
// All operations O(1) time; O(K) space for K distinct keys.
package main

import "fmt"

type Bucket struct {
	count      int64
	keys       map[string]struct{}
	prev, next *Bucket
}

func newBucket(count int64) *Bucket {
	return &Bucket{count: count, keys: make(map[string]struct{})}
}

type AllOne struct {
	head, tail *Bucket
	keyBucket  map[string]*Bucket
}

func NewAllOne() *AllOne {
	const minI = int64(-1) << 62
	const maxI = int64(1) << 62
	head := newBucket(minI)
	tail := newBucket(maxI)
	head.next = tail
	tail.prev = head
	return &AllOne{head: head, tail: tail, keyBucket: make(map[string]*Bucket)}
}

func (a *AllOne) insertAfter(node *Bucket, count int64) *Bucket {
	b := newBucket(count)
	b.prev = node
	b.next = node.next
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
	if cur, ok := a.keyBucket[key]; ok {
		newCount := cur.count + 1
		nxt := cur.next
		if nxt.count != newCount {
			nxt = a.insertAfter(cur, newCount)
		}
		nxt.keys[key] = struct{}{}
		a.keyBucket[key] = nxt
		delete(cur.keys, key)
		if len(cur.keys) == 0 {
			a.remove(cur)
		}
	} else {
		first := a.head.next
		if first.count != 1 {
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
		delete(cur.keys, key)
		delete(a.keyBucket, key)
		if len(cur.keys) == 0 {
			a.remove(cur)
		}
		return
	}
	newCount := cur.count - 1
	prv := cur.prev
	if prv.count != newCount {
		prv = a.insertAfter(cur.prev, newCount)
	}
	prv.keys[key] = struct{}{}
	a.keyBucket[key] = prv
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
	ao := NewAllOne()
	ao.Plus("a")
	ao.Plus("b")
	ao.Plus("b")
	fmt.Println("get_max: " + ao.GetMax()) // b
	fmt.Println("get_min: " + ao.GetMin()) // a
	ao.Minus("b")
	ao.Minus("b")
	fmt.Println("get_max: " + ao.GetMax()) // a
}
