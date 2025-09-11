// Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
// Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).
import java.util.*;

public class Solution {
    static String w1="SEND", w2="MORE", w3="MONEY";
    static char[] order;
    static int[] assign;
    static boolean[] used = new boolean[10];
    static Set<Character> leading = new HashSet<>();

    public static void main(String[] args) {
        LinkedHashSet<Character> seen = new LinkedHashSet<>();
        for (char c : (w1+w2+w3).toCharArray()) seen.add(c);
        order = new char[seen.size()];
        int i=0; for(char c: seen) order[i++]=c;
        assign = new int[order.length];
        leading.add(w1.charAt(0)); leading.add(w2.charAt(0)); leading.add(w3.charAt(0));

        dfs(0);

        StringBuilder sb = new StringBuilder("{");
        for (int j=0;j<order.length;j++){
            sb.append("'").append(order[j]).append("': ").append(assign[j]);
            if (j+1<order.length) sb.append(", ");
        }
        sb.append("}");
        System.out.println(sb.toString());
    }

    static long num(String w, Map<Character,Integer> val){
        long n=0; for(char c: w.toCharArray()) n=n*10+val.get(c); return n;
    }

    static boolean dfs(int idx){
        if (idx==order.length){
            Map<Character,Integer> val=new HashMap<>();
            for(int i=0;i<order.length;i++) val.put(order[i], assign[i]);
            return num(w1,val)+num(w2,val)==num(w3,val);
        }
        for(int d=0; d<10; d++){
            if(used[d]) continue;
            if(d==0 && leading.contains(order[idx])) continue;
            used[d]=true; assign[idx]=d;
            if(dfs(idx+1)) return true;
            used[d]=false; assign[idx]=-1;
        }
        return false;
    }
}
