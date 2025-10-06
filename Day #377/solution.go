// Sliding window median via two heaps with lazy deletion.
// Time: O(n log k), Space: O(k).
package main

import (
	"container/heap"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type MaxHeap []int

func (h MaxHeap) Len() int            { return len(h) }
func (h MaxHeap) Less(i, j int) bool  { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x any)         { *h = append(*h, x.(int)) }
func (h *MaxHeap) Pop() any           { old := *h; n := len(old); x := old[n-1]; *h = old[:n-1]; return x }

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *MinHeap) Pop() any          { old := *h; n := len(old); x := old[n-1]; *h = old[:n-1]; return x }

type DualHeap struct {
	small   *MaxHeap
	large   *MinHeap
	delayed map[int]int
	ss, ls  int
}

func NewDualHeap() *DualHeap {
	return &DualHeap{small: &MaxHeap{}, large: &MinHeap{}, delayed: map[int]int{}}
}
func (d *DualHeap) pruneSmall() {
	for d.small.Len() > 0 {
		num := (*d.small)[0]
		if d.delayed[num] > 0 {
			d.delayed[num]--
			if d.delayed[num] == 0 {
				delete(d.delayed, num)
			}
			heap.Pop(d.small)
		} else {
			break
		}
	}
}
func (d *DualHeap) pruneLarge() {
	for d.large.Len() > 0 {
		num := (*d.large)[0]
		if d.delayed[num] > 0 {
			d.delayed[num]--
			if d.delayed[num] == 0 {
				delete(d.delayed, num)
			}
			heap.Pop(d.large)
		} else {
			break
		}
	}
}
func (d *DualHeap) balance() {
	if d.ss > d.ls+1 {
		heap.Push(d.large, heap.Pop(d.small).(int))
		d.ss--
		d.ls++
		d.pruneSmall()
	} else if d.ss < d.ls {
		heap.Push(d.small, heap.Pop(d.large).(int))
		d.ls--
		d.ss++
		d.pruneLarge()
	}
}
func (d *DualHeap) insert(num int) {
	if d.small.Len() == 0 || num <= (*d.small)[0] {
		heap.Push(d.small, num)
		d.ss++
	} else {
		heap.Push(d.large, num)
		d.ls++
	}
	d.balance()
}
func (d *DualHeap) erase(num int) {
	d.delayed[num]++
	if num <= (*d.small)[0] {
		d.ss--
		if num == (*d.small)[0] {
			d.pruneSmall()
		}
	} else {
		d.ls--
		if num == (*d.large)[0] {
			d.pruneLarge()
		}
	}
	d.balance()
}
func (d *DualHeap) median(k int) float64 {
	if k%2 == 1 {
		return float64((*d.small)[0])
	}
	return (float64((*d.small)[0]) + float64((*d.large)[0])) / 2.0
}

func fmtNum(x float64) string {
	if x == math.Trunc(x) {
		return strconv.FormatInt(int64(x), 10)
	}
	return strconv.FormatFloat(x, 'g', -1, 64)
}

func main() {
	arr := []int{-1, 5, 13, 8, 2, 3, 3, 1}
	k := 3
	dh := NewDualHeap()
	for i := 0; i < k; i++ {
		dh.insert(arr[i])
	}
	n := len(arr)
	for i := k; i <= n; i++ {
		parts := make([]string, 0, k)
		for j := i - k; j < i; j++ {
			parts = append(parts, strconv.Itoa(arr[j]))
		}
		fmt.Printf("%s <- median of [%s]\n", fmtNum(dh.median(k)), strings.Join(parts, ", "))
		if i < n {
			dh.insert(arr[i])
			dh.erase(arr[i-k])
		}
	}
}
