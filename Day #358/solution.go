// All O(1) structure: doubly-linked list of count-buckets (increasing), each holds a key set; map key->bucket.
// plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket; rep() picks the min key on demand.
package main

import "fmt"

type Bucket struct {
	count      int
	keys       map[string]bool
	prev, next *Bucket
}

func newBucket(count int) *Bucket {
	return &Bucket{count: count, keys: make(map[string]bool)}
}

func (b *Bucket) add(key string) {
	b.keys[key] = true
}

func (b *Bucket) remove(key string) {
	delete(b.keys, key)
}

func (b *Bucket) rep() string {
	best := ""
	for k := range b.keys {
		if best == "" || k < best {
			best = k
		}
	}
	return best
}

type AllOne struct {
	head, tail *Bucket
	keyBucket  map[string]*Bucket
}

func NewAllOne() *AllOne {
	h := newBucket(-1 << 60)
	t := newBucket(1 << 60)
	h.next = t
	t.prev = h
	return &AllOne{head: h, tail: t, keyBucket: make(map[string]*Bucket)}
}

func (a *AllOne) insertAfter(b *Bucket, count int) *Bucket {
	nb := newBucket(count)
	nb.prev = b
	nb.next = b.next
	b.next.prev = nb
	b.next = nb
	return nb
}

func (a *AllOne) removeBucket(b *Bucket) {
	b.prev.next = b.next
	b.next.prev = b.prev
}

func (a *AllOne) Plus(key string) {
	if cur, ok := a.keyBucket[key]; !ok {
		first := a.head.next
		if first == a.tail || first.count != 1 {
			first = a.insertAfter(a.head, 1)
		}
		first.add(key)
		a.keyBucket[key] = first
	} else {
		nxt := cur.next
		if nxt == a.tail || nxt.count != cur.count+1 {
			nxt = a.insertAfter(cur, cur.count+1)
		}
		nxt.add(key)
		a.keyBucket[key] = nxt
		cur.remove(key)
		if len(cur.keys) == 0 {
			a.removeBucket(cur)
		}
	}
}

func (a *AllOne) Minus(key string) {
	cur, ok := a.keyBucket[key]
	if !ok {
		return
	}
	if cur.count == 1 {
		cur.remove(key)
		delete(a.keyBucket, key)
		if len(cur.keys) == 0 {
			a.removeBucket(cur)
		}
		return
	}
	prv := cur.prev
	if prv == a.head || prv.count != cur.count-1 {
		prv = a.insertAfter(cur.prev, cur.count-1)
	}
	prv.add(key)
	a.keyBucket[key] = prv
	cur.remove(key)
	if len(cur.keys) == 0 {
		a.removeBucket(cur)
	}
}

func (a *AllOne) GetMax() string {
	if a.tail.prev == a.head {
		return ""
	}
	return a.tail.prev.rep()
}

func (a *AllOne) GetMin() string {
	if a.head.next == a.tail {
		return ""
	}
	return a.head.next.rep()
}

func main() {
	a := NewAllOne()
	a.Plus("a")
	a.Plus("b")
	a.Plus("b")
	a.Plus("c")
	a.Plus("c")
	a.Plus("c")
	fmt.Println("max=" + a.GetMax())
	fmt.Println("min=" + a.GetMin())
	a.Minus("c")
	a.Minus("c")
	fmt.Println("max=" + a.GetMax())
}
