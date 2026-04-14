// XOR linked list simulated with array-backed memory & integer addresses (no real pointers).
// both = prevIdx ^ nextIdx, NULL sentinel = 0. add=O(1), get(i)=O(i) time, O(n) space.
package main

import "fmt"

const NULL = 0

type XorList struct {
	value []int
	both  []int
	head  int
	tail  int
}

func NewXorList() *XorList {
	return &XorList{value: []int{0}, both: []int{0}, head: NULL, tail: NULL} // index 0 = NULL
}

func (l *XorList) add(v int) {
	idx := len(l.value)
	l.value = append(l.value, v)
	l.both = append(l.both, 0)
	if l.head == NULL {
		l.head, l.tail = idx, idx
		return
	}
	l.both[idx] = l.tail ^ NULL // prev=tail, next=NULL
	l.both[l.tail] ^= idx       // append as next of old tail
	l.tail = idx
}

func (l *XorList) get(index int) int {
	prev, cur := NULL, l.head
	for i := 0; i < index; i++ {
		next := l.both[cur] ^ prev
		prev = cur
		cur = next
	}
	return l.value[cur]
}

func main() {
	list := NewXorList()
	for _, x := range []int{10, 20, 30, 40} {
		list.add(x)
	}
	for i := 0; i < 4; i++ {
		fmt.Println(list.get(i))
	}
}
