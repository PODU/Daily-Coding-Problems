// Day 366: Rearrange so no two adjacent chars match (else null).
// Greedy with a max-heap by frequency; always place the most frequent char that
// isn't the one just placed. Feasible iff maxFreq <= (n+1)/2. Time O(n log k).
package main

import (
	"container/heap"
	"fmt"
)

type item struct {
	ch  byte
	cnt int
}
type maxHeap []item

func (h maxHeap) Len() int            { return len(h) }
func (h maxHeap) Less(i, j int) bool  { return h[i].cnt > h[j].cnt }
func (h maxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *maxHeap) Push(x interface{}) { *h = append(*h, x.(item)) }
func (h *maxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func reorganize(s string) string {
	cnt := map[byte]int{}
	for i := 0; i < len(s); i++ {
		cnt[s[i]]++
	}
	h := &maxHeap{}
	for ch, c := range cnt {
		heap.Push(h, item{ch, c})
	}
	res := make([]byte, 0, len(s))
	prev := item{0, 0}
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		res = append(res, cur.ch)
		if prev.cnt > 0 {
			heap.Push(h, prev)
		}
		cur.cnt--
		prev = cur
	}
	if len(res) == len(s) {
		return string(res)
	}
	return "null"
}

func main() {
	fmt.Println(reorganize("yyz")) // yzy
	fmt.Println(reorganize("yyy")) // null
}
