// Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
// Recursion over nested LinkedHashMaps preserves order. Time/Space O(total keys).
import java.util.*;

public class Solution {
    @SuppressWarnings("unchecked")
    static void flatten(Map<String, Object> map, String prefix, Map<String, Object> out) {
        for (var e : map.entrySet()) {
            String key = prefix.isEmpty() ? e.getKey() : prefix + "." + e.getKey();
            if (e.getValue() instanceof Map) flatten((Map<String, Object>) e.getValue(), key, out);
            else out.put(key, e.getValue());
        }
    }

    public static void main(String[] args) {
        Map<String, Object> bar = new LinkedHashMap<>(); bar.put("baz", 8);
        Map<String, Object> foo = new LinkedHashMap<>(); foo.put("a", 5); foo.put("bar", bar);
        Map<String, Object> root = new LinkedHashMap<>(); root.put("key", 3); root.put("foo", foo);

        Map<String, Object> out = new LinkedHashMap<>();
        flatten(root, "", out);
        StringBuilder sb = new StringBuilder("{\n");
        int i = 0;
        for (var e : out.entrySet())
            sb.append("    \"").append(e.getKey()).append("\": ").append(e.getValue())
              .append(++i < out.size() ? "," : "").append("\n");
        sb.append("}");
        System.out.println(sb);
    }
}
