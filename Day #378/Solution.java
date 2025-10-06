// Custom JSON serializer for null/number/string/list/dict (recursive).
// Time: O(total size), Space: O(depth).
import java.util.*;

public class Solution {
    static String esc(String s){
        StringBuilder b = new StringBuilder("\"");
        for(char c : s.toCharArray()){ if(c=='"'||c=='\\') b.append('\\'); b.append(c); }
        return b.append('"').toString();
    }

    @SuppressWarnings("unchecked")
    static String encode(Object o){
        if(o == null) return "null";
        if(o instanceof String) return esc((String)o);
        if(o instanceof Boolean) return o.toString();
        if(o instanceof Number) return o.toString();
        if(o instanceof List){
            StringBuilder b = new StringBuilder("[");
            List<Object> l = (List<Object>)o;
            for(int i=0;i<l.size();i++){ if(i>0) b.append(", "); b.append(encode(l.get(i))); }
            return b.append("]").toString();
        }
        if(o instanceof Map){
            StringBuilder b = new StringBuilder("{");
            boolean first = true;
            for(Map.Entry<String,Object> e : ((Map<String,Object>)o).entrySet()){
                if(!first) b.append(", ");
                first = false;
                b.append(esc(e.getKey())).append(": ").append(encode(e.getValue()));
            }
            return b.append("}").toString();
        }
        return "null";
    }

    public static void main(String[] args){
        Map<String,Object> m = new LinkedHashMap<>();
        m.put("c", "d");
        List<Object> data = Arrays.asList(null, 123, Arrays.asList("a","b"), m);
        System.out.println("'" + encode(data) + "'");
    }
}
