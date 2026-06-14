// Fisher-Yates shuffle of linked-list nodes via slice: O(n) time, O(n) space.
// Space-over-time tradeoff: O(1)-extra approach repeatedly picks a random node by traversal in O(n^2) time.
package main

import (
	"fmt"
	"math/rand"
	"sort"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func main() {
	var head, tail *Node
	for v := 1; v <= 5; v++ {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	var a []*Node
	for p := head; p != nil; p = p.next {
		a = append(a, p)
	}
	n := len(a)
	rng := rand.New(rand.NewSource(12345))
	for i := n - 1; i > 0; i-- {
		j := rng.Intn(i + 1)
		a[i], a[j] = a[j], a[i]
	}
	for i := 0; i < n; i++ {
		if i+1 < n {
			a[i].next = a[i+1]
		} else {
			a[i].next = nil
		}
	}
	head = a[0]
	orig := []int{1, 2, 3, 4, 5}
	var shuf []int
	for p := head; p != nil; p = p.next {
		shuf = append(shuf, p.val)
	}
	sort.Ints(shuf)
	valid := true
	for i := range orig {
		if orig[i] != shuf[i] {
			valid = false
		}
	}
	parts := make([]string, len(orig))
	for i, v := range orig {
		parts[i] = fmt.Sprintf("%d", v)
	}
	msg := "valid shuffle (same elements)"
	if !valid {
		msg = "INVALID"
	}
	fmt.Println(strings.Join(parts, " ") + " -> " + msg)
}
