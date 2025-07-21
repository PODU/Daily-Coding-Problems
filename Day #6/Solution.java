// XOR linked list simulated with a "memory" array; addresses are indices (0 = null).
// each node stores both = prevAddr XOR nextAddr. add: O(1), get(i): O(i). Space: O(n).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        int both; // prevAddr XOR nextAddr
        Node(int v) { val = v; }
    }

    static class XorList {
        List<Node> mem = new ArrayList<>();
        int head = 0, tail = 0; // 0 = null sentinel

        XorList() { mem.add(null); } // index 0 reserved as null

        int alloc(Node n) { mem.add(n); return mem.size() - 1; }

        void add(int val) {
            int addr = alloc(new Node(val));
            if (head == 0) { head = tail = addr; return; }
            mem.get(tail).both ^= addr;   // tail.next = addr
            mem.get(addr).both = tail;     // node.prev = tail
            tail = addr;
        }

        Node get(int index) {
            int prev = 0, cur = head;
            for (int i = 0; i < index && cur != 0; i++) {
                int next = mem.get(cur).both ^ prev;
                prev = cur;
                cur = next;
            }
            return cur == 0 ? null : mem.get(cur);
        }
    }

    public static void main(String[] args) {
        XorList l = new XorList();
        for (int v : new int[]{10, 20, 30, 40}) l.add(v);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 4; i++) sb.append(l.get(i).val).append(i < 3 ? " " : "");
        System.out.println(sb);
    }
}
