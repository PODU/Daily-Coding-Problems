// Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
// Recursion over nested maps; ordered keys for stable output. Time/Space O(total keys).
package main

import "fmt"

// orderedDict keeps keys in insertion order for deterministic flattening.
type entry struct {
	key string
	val interface{}
}
type od []entry

func flatten(d od, prefix string, out *od) {
	for _, e := range d {
		key := e.key
		if prefix != "" {
			key = prefix + "." + e.key
		}
		if sub, ok := e.val.(od); ok {
			flatten(sub, key, out)
		} else {
			*out = append(*out, entry{key, e.val})
		}
	}
}

func main() {
	bar := od{{"baz", 8}}
	foo := od{{"a", 5}, {"bar", bar}}
	root := od{{"key", 3}, {"foo", foo}}

	var out od
	flatten(root, "", &out)
	fmt.Println("{")
	for i, e := range out {
		comma := ","
		if i+1 == len(out) {
			comma = ""
		}
		fmt.Printf("    \"%s\": %v%s\n", e.key, e.val, comma)
	}
	fmt.Println("}")
}
