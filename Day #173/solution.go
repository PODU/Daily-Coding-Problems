// Flatten nested dictionary, namespacing keys with '.'. Recursive DFS preserving insertion order.
// Time O(total keys), Space O(total keys).
package main

import "fmt"

// entry preserves insertion order for deterministic output.
type entry struct {
	key   string
	value interface{}
}

type omap struct{ items []entry }

func (m *omap) put(k string, v interface{}) { m.items = append(m.items, entry{k, v}) }

func flatten(m *omap, prefix string, out *omap) {
	for _, e := range m.items {
		key := e.key
		if prefix != "" {
			key = prefix + "." + e.key
		}
		if nested, ok := e.value.(*omap); ok {
			flatten(nested, key, out)
		} else {
			out.put(key, e.value)
		}
	}
}

func main() {
	bar := &omap{}
	bar.put("baz", 8)
	foo := &omap{}
	foo.put("a", 5)
	foo.put("bar", bar)
	root := &omap{}
	root.put("key", 3)
	root.put("foo", foo)

	out := &omap{}
	flatten(root, "", out)
	for _, e := range out.items {
		fmt.Printf("%s: %v\n", e.key, e.value)
	}
}
