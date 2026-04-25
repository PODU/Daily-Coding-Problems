// Day 1426: Maximum XOR of any two elements in an array.
// Approach: Binary trie of bits; for each number greedily pick opposite bit.
// Time: O(n * B), Space: O(n * B) where B = number of bits.
package main

import "fmt"

type trie struct {
	child [2]*trie
}

func findMaxXOR(nums []int) int {
	root := &trie{}
	maxXor := 0
	const BITS = 31
	for _, num := range nums {
		node := root
		for b := BITS; b >= 0; b-- {
			bit := (num >> b) & 1
			if node.child[bit] == nil {
				node.child[bit] = &trie{}
			}
			node = node.child[bit]
		}
		node = root
		cur := 0
		for b := BITS; b >= 0; b-- {
			bit := (num >> b) & 1
			if node.child[1-bit] != nil {
				cur |= 1 << b
				node = node.child[1-bit]
			} else {
				node = node.child[bit]
			}
		}
		if cur > maxXor {
			maxXor = cur
		}
	}
	return maxXor
}

func main() {
	fmt.Println(findMaxXOR([]int{3, 10, 5, 25, 2, 8})) // 28
}
