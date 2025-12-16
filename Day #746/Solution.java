// Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
// Time: O(1) per operation, Space: O(n).
import java.util.*;

public class Solution {
    static class MaxStack {
        Deque<int[]> st = new ArrayDeque<>(); // {value, maxSoFar}
        void push(int v){
            int mx = st.isEmpty() ? v : Math.max(v, st.peek()[1]);
            st.push(new int[]{v, mx});
        }
        Integer pop(){
            if(st.isEmpty()) return null;
            return st.pop()[0];
        }
        Integer max(){
            if(st.isEmpty()) return null;
            return st.peek()[1];
        }
    }

    public static void main(String[] args){
        MaxStack s = new MaxStack();
        s.push(1); s.push(3); s.push(2);
        System.out.println("max: " + s.max()); // 3
        System.out.println("pop: " + s.pop()); // 2
        System.out.println("max: " + s.max()); // 3
        System.out.println("pop: " + s.pop()); // 3
        System.out.println("max: " + s.max()); // 1
    }
}
