// Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
// Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).
package main

import "fmt"

type Trie struct {
	child [2]*Trie
}

func insertNum(root *Trie, num int) {
	node := root
	for i := 31; i >= 0; i-- {
		b := (num >> uint(i)) & 1
		if node.child[b] == nil {
			node.child[b] = &Trie{}
		}
		node = node.child[b]
	}
}

func queryBest(root *Trie, num int) int {
	node := root
	best := 0
	for i := 31; i >= 0; i-- {
		b := (num >> uint(i)) & 1
		want := b ^ 1
		if node.child[want] != nil {
			best |= 1 << uint(i)
			node = node.child[want]
		} else {
			node = node.child[b]
		}
	}
	return best
}

func findMaximumXOR(nums []int) int {
	root := &Trie{}
	best := 0
	for _, x := range nums {
		insertNum(root, x)
		if q := queryBest(root, x); q > best {
			best = q
		}
	}
	return best
}

func main() {
	nums := []int{3, 10, 5, 25, 2, 8}
	fmt.Println(findMaximumXOR(nums))
}
