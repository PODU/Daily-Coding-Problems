// Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
// Time: O(10!/(10-k)!) for k unique letters; Space: O(k).
import java.util.*;

public class Solution {
    static String A, B, C;
    static List<Character> letters = new ArrayList<>();
    static Set<Character> leading = new HashSet<>();
    static Map<Character,Integer> assign = new HashMap<>();
    static boolean[] used = new boolean[10];

    static long val(String w){ long v=0; for(char c: w.toCharArray()) v=v*10+assign.get(c); return v; }

    static boolean dfs(int i){
        if(i==letters.size()) return val(A)+val(B)==val(C);
        char c = letters.get(i);
        for(int d=0; d<10; d++){
            if(used[d]) continue;
            if(d==0 && leading.contains(c)) continue;
            used[d]=true; assign.put(c,d);
            if(dfs(i+1)) return true;
            used[d]=false;
        }
        return false;
    }

    public static void main(String[] args){
        A="SEND"; B="MORE"; C="MONEY";
        Set<Character> seen = new HashSet<>();
        for(String w : new String[]{A,B,C})
            for(char c : w.toCharArray())
                if(seen.add(c)) letters.add(c);
        leading.add(A.charAt(0)); leading.add(B.charAt(0)); leading.add(C.charAt(0));
        dfs(0);
        String order = "SENDMORY";
        StringBuilder sb = new StringBuilder("{");
        for(int i=0;i<order.length();i++){
            char c=order.charAt(i);
            sb.append("'").append(c).append("': ").append(assign.get(c));
            if(i+1<order.length()) sb.append(", ");
        }
        sb.append("}");
        System.out.println(sb);
    }
}
