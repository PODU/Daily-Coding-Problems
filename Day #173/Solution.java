// Flatten nested dictionary, namespacing keys with '.'. Recursive DFS preserving insertion order.
// Time O(total keys), Space O(total keys).
import java.util.*;

public class Solution {
    static void flatten(Map<String,Object> map, String prefix, LinkedHashMap<String,Object> out) {
        for (Map.Entry<String,Object> e : map.entrySet()) {
            String key = prefix.isEmpty() ? e.getKey() : prefix + "." + e.getKey();
            Object v = e.getValue();
            if (v instanceof Map) {
                @SuppressWarnings("unchecked")
                Map<String,Object> nested = (Map<String,Object>) v;
                flatten(nested, key, out);
            } else {
                out.put(key, v);
            }
        }
    }

    public static void main(String[] args) {
        LinkedHashMap<String,Object> root = new LinkedHashMap<>();
        root.put("key", 3);
        LinkedHashMap<String,Object> foo = new LinkedHashMap<>();
        foo.put("a", 5);
        LinkedHashMap<String,Object> bar = new LinkedHashMap<>();
        bar.put("baz", 8);
        foo.put("bar", bar);
        root.put("foo", foo);

        LinkedHashMap<String,Object> out = new LinkedHashMap<>();
        flatten(root, "", out);
        for (Map.Entry<String,Object> e : out.entrySet())
            System.out.println(e.getKey() + ": " + e.getValue());
    }
}
