// Flatten a nested dictionary, joining keys with '.' via DFS.
// LinkedHashMap preserves insertion order.
// Time: O(total keys), Space: O(depth) recursion.
import java.util.*;

public class Solution {
    @SuppressWarnings("unchecked")
    static void flatten(String prefix, Map<String,Object> map, LinkedHashMap<String,Object> out){
        for(Map.Entry<String,Object> e : map.entrySet()){
            String key = prefix.isEmpty() ? e.getKey() : prefix + "." + e.getKey();
            if(e.getValue() instanceof Map) flatten(key, (Map<String,Object>)e.getValue(), out);
            else out.put(key, e.getValue());
        }
    }

    public static void main(String[] args){
        LinkedHashMap<String,Object> bar = new LinkedHashMap<>();
        bar.put("baz", 8);
        LinkedHashMap<String,Object> foo = new LinkedHashMap<>();
        foo.put("a", 5);
        foo.put("bar", bar);
        LinkedHashMap<String,Object> root = new LinkedHashMap<>();
        root.put("key", 3);
        root.put("foo", foo);

        LinkedHashMap<String,Object> out = new LinkedHashMap<>();
        flatten("", root, out);

        StringBuilder sb = new StringBuilder("{");
        int i = 0;
        for(Map.Entry<String,Object> e : out.entrySet()){
            sb.append("'").append(e.getKey()).append("': ").append(e.getValue());
            if(++i < out.size()) sb.append(", ");
        }
        sb.append("}");
        System.out.println(sb); // {'key': 3, 'foo.a': 5, 'foo.bar.baz': 8}
    }
}
