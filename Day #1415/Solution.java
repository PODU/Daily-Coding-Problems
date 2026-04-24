// Day 1415: flatten a nested dictionary, namespacing keys with '.'.
// Approach: recursive DFS building prefixed keys. O(total keys) time/space.
import java.util.*;

public class Solution {
    @SuppressWarnings("unchecked")
    static void flatten(String prefix, Object value, LinkedHashMap<String,Integer> out) {
        if (value instanceof Map) {
            for (Map.Entry<String,Object> e : ((Map<String,Object>) value).entrySet()) {
                String key = prefix.isEmpty() ? e.getKey() : prefix + "." + e.getKey();
                flatten(key, e.getValue(), out);
            }
        } else {
            out.put(prefix, (Integer) value);
        }
    }

    public static void main(String[] args) {
        LinkedHashMap<String,Object> bar = new LinkedHashMap<>();
        bar.put("baz", 8);
        LinkedHashMap<String,Object> foo = new LinkedHashMap<>();
        foo.put("a", 5);
        foo.put("bar", bar);
        LinkedHashMap<String,Object> root = new LinkedHashMap<>();
        root.put("key", 3);
        root.put("foo", foo);

        LinkedHashMap<String,Integer> out = new LinkedHashMap<>();
        flatten("", root, out);
        for (Map.Entry<String,Integer> e : out.entrySet())
            System.out.println(e.getKey() + ": " + e.getValue());
    }
}
