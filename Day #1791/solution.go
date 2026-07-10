// Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
//
// Idea: a doubly linked list of "buckets", one per distinct count, kept
// sorted by count. Each bucket owns the set of keys sitting at that count,
// and a map points every key at its current bucket. Incrementing or
// decrementing a key only moves it to a neighbouring bucket, so each
// operation touches a constant number of nodes. Min and max are the
// buckets right next to the sentinels.
package main

import (
	"fmt"
	"math"
)

// Bucket is a node in the linked list: one count, plus every key that has it.
type Bucket struct {
	count      int
	keys       map[string]struct{}
	prev, next *Bucket
}

func newBucket(c int) *Bucket {
	return &Bucket{count: c, keys: make(map[string]struct{})}
}

type AllOne struct {
	head, tail *Bucket            // sentinels, so no front/back special cases
	keyBucket  map[string]*Bucket // key -> the bucket it currently lives in
}

func NewAllOne() *AllOne {
	h := newBucket(math.MinInt32)
	t := newBucket(math.MaxInt32)
	h.next = t
	t.prev = h
	return &AllOne{head: h, tail: t, keyBucket: make(map[string]*Bucket)}
}

// insertAfter splices a fresh bucket for count right after node.
func (a *AllOne) insertAfter(node *Bucket, count int) *Bucket {
	b := newBucket(count)
	b.prev, b.next = node, node.next
	node.next.prev = b
	node.next = b
	return b
}

// remove unlinks an empty bucket from the list.
func (a *AllOne) remove(b *Bucket) {
	b.prev.next = b.next
	b.next.prev = b.prev
}

func (a *AllOne) Plus(key string) {
	if cur, ok := a.keyBucket[key]; ok {
		// Existing key: shift it one bucket to the right (count + 1).
		nxt := cur.next
		if nxt == a.tail || nxt.count != cur.count+1 {
			// No bucket for count+1 yet, so make one next door.
			nxt = a.insertAfter(cur, cur.count+1)
		}
		nxt.keys[key] = struct{}{}
		a.keyBucket[key] = nxt
		delete(cur.keys, key)
		if len(cur.keys) == 0 {
			a.remove(cur) // the bucket we left is empty now
		}
	} else {
		// New key: it belongs in the count-1 bucket at the front.
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
		return // unknown key -> no-op
	}

	delete(cur.keys, key)

	if cur.count == 1 {
		// Count would hit zero, so the key disappears entirely.
		delete(a.keyBucket, key)
	} else {
		// Shift the key one bucket to the left (count - 1).
		prv := cur.prev
		if prv == a.head || prv.count != cur.count-1 {
			prv = a.insertAfter(cur.prev, cur.count-1)
		}
		prv.keys[key] = struct{}{}
		a.keyBucket[key] = prv
	}

	if len(cur.keys) == 0 {
		a.remove(cur)
	}
}

// smallestKey picks the lexicographically smallest key in a bucket. Any key
// would be a valid answer; this just keeps the output deterministic.
func smallestKey(b *Bucket) string {
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
	return smallestKey(a.tail.prev)
}

func (a *AllOne) GetMin() string {
	if a.head.next == a.tail {
		return ""
	}
	return smallestKey(a.head.next)
}

func main() {
	a := NewAllOne()
	a.Plus("apple")
	a.Plus("apple")
	a.Plus("banana")
	fmt.Printf("max=%s min=%s\n", a.GetMax(), a.GetMin()) // apple / banana

	a.Plus("banana")
	a.Plus("banana")
	fmt.Printf("max=%s min=%s\n", a.GetMax(), a.GetMin()) // banana / apple

	a.Minus("apple")
	a.Minus("apple")
	fmt.Printf("max=%s min=%s\n", a.GetMax(), a.GetMin()) // banana / banana
}
