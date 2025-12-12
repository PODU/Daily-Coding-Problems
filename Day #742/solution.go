// Flatten a nested dictionary, joining keys with '.' via DFS.
// Uses ordered key slices to preserve insertion order.
// Time: O(total keys), Space: O(depth) recursion.
package main

import (
	"fmt"
	"strings"
)

// Dict is an ordered map: keys give order, values may be int or nested *Dict.
type Dict struct {
	keys []string
	vals map[string]interface{}
}

func NewDict() *Dict { return &Dict{vals: map[string]interface{}{}} }
func (d *Dict) Set(k string, v interface{}) {
	d.keys = append(d.keys, k)
	d.vals[k] = v
}

func flatten(prefix string, d *Dict, outKeys *[]string, outVals map[string]int) {
	for _, k := range d.keys {
		key := k
		if prefix != "" {
			key = prefix + "." + k
		}
		switch v := d.vals[k].(type) {
		case *Dict:
			flatten(key, v, outKeys, outVals)
		case int:
			*outKeys = append(*outKeys, key)
			outVals[key] = v
		}
	}
}

func main() {
	bar := NewDict()
	bar.Set("baz", 8)
	foo := NewDict()
	foo.Set("a", 5)
	foo.Set("bar", bar)
	root := NewDict()
	root.Set("key", 3)
	root.Set("foo", foo)

	var outKeys []string
	outVals := map[string]int{}
	flatten("", root, &outKeys, outVals)

	parts := make([]string, len(outKeys))
	for i, k := range outKeys {
		parts[i] = fmt.Sprintf("'%s': %d", k, outVals[k])
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
	// {'key': 3, 'foo.a': 5, 'foo.bar.baz': 8}
}
