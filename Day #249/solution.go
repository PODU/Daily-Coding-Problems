// Maximum XOR of any two elements using a binary trie of bits.
// Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.
package main

import "fmt"

const BITS = 32

type Trie struct {
	child [2]*Trie
}

func findMaxXOR(nums []int) int {
	root := &Trie{}
	for _, x := range nums {
		node := root
		for i := BITS - 1; i >= 0; i-- {
			b := (x >> uint(i)) & 1
			if node.child[b] == nil {
				node.child[b] = &Trie{}
			}
			node = node.child[b]
		}
	}
	best := 0
	for _, x := range nums {
		node := root
		cur := 0
		for i := BITS - 1; i >= 0; i-- {
			b := (x >> uint(i)) & 1
			want := b ^ 1
			if node.child[want] != nil {
				cur |= 1 << uint(i)
				node = node.child[want]
			} else {
				node = node.child[b]
			}
		}
		if cur > best {
			best = cur
		}
	}
	return best
}

func main() {
	nums := []int{3, 10, 5, 25, 2, 8}
	fmt.Println(findMaxXOR(nums))
}
