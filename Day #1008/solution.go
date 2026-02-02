// Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
// Time O(n*B), Space O(n*B), B = 31.
package main

import "fmt"

const B = 31

type node struct {
	child [2]*node
}

func maximumXOR(nums []int) int {
	root := &node{}
	best := 0
	for _, x := range nums {
		ins, cur := root, root
		curXor := 0
		for b := B - 1; b >= 0; b-- {
			bit := (x >> uint(b)) & 1
			if ins.child[bit] == nil {
				ins.child[bit] = &node{}
			}
			ins = ins.child[bit]
			want := bit ^ 1
			if cur.child[want] != nil {
				curXor |= 1 << uint(b)
				cur = cur.child[want]
			} else if cur.child[bit] != nil {
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
	fmt.Println(maximumXOR(nums)) // 28
}
