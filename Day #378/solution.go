// Custom JSON serializer for null/number/string/list/dict (recursive).
// Time: O(total size), Space: O(depth).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

// pair preserves insertion order for dictionaries.
type pair struct {
	k string
	v interface{}
}
type odict []pair

func esc(s string) string {
	s = strings.ReplaceAll(s, "\\", "\\\\")
	s = strings.ReplaceAll(s, "\"", "\\\"")
	return "\"" + s + "\""
}

func encode(o interface{}) string {
	switch v := o.(type) {
	case nil:
		return "null"
	case bool:
		if v {
			return "true"
		}
		return "false"
	case int:
		return strconv.Itoa(v)
	case float64:
		return strconv.FormatFloat(v, 'g', -1, 64)
	case string:
		return esc(v)
	case []interface{}:
		parts := make([]string, len(v))
		for i, x := range v {
			parts[i] = encode(x)
		}
		return "[" + strings.Join(parts, ", ") + "]"
	case odict:
		parts := make([]string, len(v))
		for i, p := range v {
			parts[i] = esc(p.k) + ": " + encode(p.v)
		}
		return "{" + strings.Join(parts, ", ") + "}"
	}
	return "null"
}

func main() {
	data := []interface{}{
		nil,
		123,
		[]interface{}{"a", "b"},
		odict{{"c", "d"}},
	}
	fmt.Println("'" + encode(data) + "'")
}
