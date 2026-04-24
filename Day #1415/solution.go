// Day 1415: flatten a nested dictionary, namespacing keys with '.'.
// Approach: recursive DFS building prefixed keys. O(total keys) time/space.
package main

import "fmt"

// Pair preserves insertion order for the demo.
type Pair struct {
	Key string
	Val interface{} // int or []Pair
}

func flatten(obj []Pair, prefix string, out *[]Pair) {
	for _, p := range obj {
		key := p.Key
		if prefix != "" {
			key = prefix + "." + p.Key
		}
		if nested, ok := p.Val.([]Pair); ok {
			flatten(nested, key, out)
		} else {
			*out = append(*out, Pair{key, p.Val})
		}
	}
}

func main() {
	data := []Pair{
		{"key", 3},
		{"foo", []Pair{
			{"a", 5},
			{"bar", []Pair{{"baz", 8}}},
		}},
	}
	var out []Pair
	flatten(data, "", &out)
	for _, p := range out {
		fmt.Printf("%s: %v\n", p.Key, p.Val)
	}
}
