// Flatten nested dict, namespacing keys with '.'. Recurse; on nested ordered map
// prepend parentKey + '.'. Insertion order kept via slice of keys.
// Time O(total entries), Space O(depth).
package main

import (
	"fmt"
	"strings"
)

// OMap is an ordered map: parallel slices of keys and values.
type OMap struct {
	keys []string
	vals []interface{}
}

func (m *OMap) set(k string, v interface{}) {
	m.keys = append(m.keys, k)
	m.vals = append(m.vals, v)
}

func flatten(m *OMap, prefix string, out *OMap) {
	for i, k := range m.keys {
		key := prefix + k
		if nested, ok := m.vals[i].(*OMap); ok {
			flatten(nested, key+".", out)
		} else {
			out.set(key, m.vals[i])
		}
	}
}

func main() {
	// {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
	bar := &OMap{}
	bar.set("baz", 8)
	foo := &OMap{}
	foo.set("a", 5)
	foo.set("bar", bar)
	data := &OMap{}
	data.set("key", 3)
	data.set("foo", foo)

	out := &OMap{}
	flatten(data, "", out)

	parts := make([]string, len(out.keys))
	for i, k := range out.keys {
		parts[i] = fmt.Sprintf("\"%s\": %v", k, out.vals[i])
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
