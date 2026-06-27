// Sort a k-sorted array (each element <= k from its sorted position).
// Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.
package main

import (
	"container/heap"
	"fmt"
	"strings"
)

type IntHeap []int

func (h IntHeap) Len() int            { return len(h) }
func (h IntHeap) Less(i, j int) bool  { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	v := old[n-1]
	*h = old[:n-1]
	return v
}

func sortKSorted(arr []int, k int) []int {
	mh := &IntHeap{}
	heap.Init(mh)
	result := make([]int, 0, len(arr))
	i := 0
	for ; i <= k && i < len(arr); i++ {
		heap.Push(mh, arr[i])
	}
	for ; i < len(arr); i++ {
		result = append(result, heap.Pop(mh).(int))
		heap.Push(mh, arr[i])
	}
	for mh.Len() > 0 {
		result = append(result, heap.Pop(mh).(int))
	}
	return result
}

func main() {
	arr := []int{2, 1, 4, 3, 6, 5} // k-sorted with k = 2
	sorted := sortKSorted(arr, 2)
	parts := make([]string, len(sorted))
	for i, v := range sorted {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
