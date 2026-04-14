// XOR linked list simulated with array-backed memory & integer addresses (no real pointers).
// both = prevIdx ^ nextIdx, NULL sentinel = 0. add=O(1), get(i)=O(i) time, O(n) space.
import java.util.ArrayList;

public class Solution {
    static final int NULL = 0;
    ArrayList<Integer> value = new ArrayList<>();
    ArrayList<Integer> both = new ArrayList<>();
    int head = NULL, tail = NULL;

    Solution() { value.add(0); both.add(0); } // reserve index 0 as NULL

    void add(int v) {
        int idx = value.size();
        value.add(v);
        both.add(0);
        if (head == NULL) { head = tail = idx; return; }
        both.set(idx, tail ^ NULL);            // prev=tail, next=NULL
        both.set(tail, both.get(tail) ^ idx);  // append as next of old tail
        tail = idx;
    }

    int get(int index) {
        int prev = NULL, cur = head;
        for (int i = 0; i < index; i++) {
            int next = both.get(cur) ^ prev;
            prev = cur;
            cur = next;
        }
        return value.get(cur);
    }

    public static void main(String[] args) {
        Solution list = new Solution();
        list.add(10); list.add(20); list.add(30); list.add(40);
        System.out.println(list.get(0));
        System.out.println(list.get(1));
        System.out.println(list.get(2));
        System.out.println(list.get(3));
    }
}
