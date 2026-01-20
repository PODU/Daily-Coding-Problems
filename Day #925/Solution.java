// Flatten nested dict, namespacing keys with '.'. Recurse; on nested Map prepend
// parentKey + '.'. LinkedHashMap preserves insertion order.
// Time O(total entries), Space O(depth).
import java.util.LinkedHashMap;
import java.util.Map;

public class Solution {
    @SuppressWarnings("unchecked")
    static void flatten(Map<String, Object> d, String prefix,
                        LinkedHashMap<String, Object> out) {
        for (Map.Entry<String, Object> e : d.entrySet()) {
            String key = prefix + e.getKey();
            if (e.getValue() instanceof Map) {
                flatten((Map<String, Object>) e.getValue(), key + ".", out);
            } else {
                out.put(key, e.getValue());
            }
        }
    }

    public static void main(String[] args) {
        LinkedHashMap<String, Object> data = new LinkedHashMap<>();
        data.put("key", 3);
        LinkedHashMap<String, Object> foo = new LinkedHashMap<>();
        foo.put("a", 5);
        LinkedHashMap<String, Object> bar = new LinkedHashMap<>();
        bar.put("baz", 8);
        foo.put("bar", bar);
        data.put("foo", foo);

        LinkedHashMap<String, Object> out = new LinkedHashMap<>();
        flatten(data, "", out);

        StringBuilder sb = new StringBuilder("{");
        boolean first = true;
        for (Map.Entry<String, Object> e : out.entrySet()) {
            if (!first) sb.append(", ");
            sb.append("\"").append(e.getKey()).append("\": ").append(e.getValue());
            first = false;
        }
        sb.append("}");
        System.out.println(sb.toString());
    }
}
