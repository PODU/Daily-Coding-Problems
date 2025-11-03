// Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
// Time O(n*32), Space O(n*32).
package main

import "fmt"

type Trie struct {
	child [2]*Trie
}

func insert(root *Trie, num int) {
	cur := root
	for b := 31; b >= 0; b-- {
		bit := (num >> b) & 1
		if cur.child[bit] == nil {
			cur.child[bit] = &Trie{}
		}
		cur = cur.child[bit]
	}
}

func maxXor(nums []int) int {
	root := &Trie{}
	for _, x := range nums {
		insert(root, x)
	}
	best := 0
	for _, x := range nums {
		cur := root
		curXor := 0
		for b := 31; b >= 0; b-- {
			bit := (x >> b) & 1
			want := bit ^ 1
			if cur.child[want] != nil {
				curXor |= 1 << b
				cur = cur.child[want]
			} else {
				cur = cur.child[bit]
			}
		}
		if curXor > best {
			best = curXor
		}
	}
	return best
}

func main() {
	nums := []int{3, 10, 5, 25, 2, 8}
	fmt.Println(maxXor(nums))
}
