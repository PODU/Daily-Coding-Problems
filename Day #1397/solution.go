// Two-pointer: advance pa/pb; on reaching end switch to other head.
// They meet at intersection after at most M+N steps. Time O(M+N), Space O(1).
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func getIntersection(a, b *Node) *Node {
	if a == nil || b == nil {
		return nil
	}
	pa, pb := a, b
	for pa != pb {
		if pa == nil {
			pa = b
		} else {
			pa = pa.next
		}
		if pb == nil {
			pb = a
		} else {
			pb = pb.next
		}
	}
	return pa
}

func main() {
	shared := &Node{8, &Node{10, nil}}
	a := &Node{3, &Node{7, shared}}
	b := &Node{99, &Node{1, shared}}
	fmt.Println("the node with value", getIntersection(a, b).val)
}
