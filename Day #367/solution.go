// Day 367: Lazily merge two sorted iterators into one sorted iterator.
// Channels model lazy iterators; we peek the head of each and emit the smaller.
// Time O(n+m), Space O(1) beyond the channels.
package main

import "fmt"

func gen(nums []int) <-chan int {
	ch := make(chan int)
	go func() {
		for _, n := range nums {
			ch <- n
		}
		close(ch)
	}()
	return ch
}

func mergeIterators(a, b <-chan int) <-chan int {
	out := make(chan int)
	go func() {
		x, okA := <-a
		y, okB := <-b
		for okA || okB {
			if !okB || (okA && x <= y) {
				out <- x
				x, okA = <-a
			} else {
				out <- y
				y, okB = <-b
			}
		}
		close(out)
	}()
	return out
}

func main() {
	foo := gen([]int{5, 10, 15})
	bar := gen([]int{3, 8, 9})
	for num := range mergeIterators(foo, bar) {
		fmt.Println(num) // 3 5 8 9 10 15
	}
}
