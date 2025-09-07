// Reorganize string: greedily place the most frequent remaining char that differs from the last.
// Max-heap by count. Time: O(n log A), Space: O(A).
package main

import (
	"container/heap"
	"fmt"
)

type item struct {
	cnt int
	ch  byte
}
type pq []item

func (p pq) Len() int { return len(p) }
func (p pq) Less(i, j int) bool { // highest count, then smallest char (deterministic)
	if p[i].cnt != p[j].cnt {
		return p[i].cnt > p[j].cnt
	}
	return p[i].ch < p[j].ch
}
func (p pq) Swap(i, j int)       { p[i], p[j] = p[j], p[i] }
func (p *pq) Push(x interface{}) { *p = append(*p, x.(item)) }
func (p *pq) Pop() interface{} {
	old := *p
	n := len(old)
	x := old[n-1]
	*p = old[:n-1]
	return x
}

func reorganize(s string) string {
	cnt := map[byte]int{}
	for i := 0; i < len(s); i++ {
		cnt[s[i]]++
	}
	h := &pq{}
	for ch, c := range cnt {
		*h = append(*h, item{c, ch})
	}
	heap.Init(h)
	res := []byte{}
	var prev *item
	for h.Len() > 0 {
		cur := heap.Pop(h).(item)
		res = append(res, cur.ch)
		if prev != nil && prev.cnt > 0 {
			heap.Push(h, *prev)
		}
		cur.cnt--
		p := cur
		prev = &p
	}
	if len(res) == len(s) {
		return string(res)
	}
	return "None"
}

func main() {
	fmt.Println(reorganize("aaabbc")) // ababac (a valid arrangement)
	fmt.Println(reorganize("aaab"))   // None
}
