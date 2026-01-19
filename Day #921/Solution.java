// XOR linked list via memory-pool simulation: nodes stored in a list, indices act as
// "addresses"; both = prevIndex XOR nextIndex (sentinel 0 = null, real nodes start at 1).
// add O(1) with tail tracking, get O(index). O(1) extra per node.
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int value;
        int both; // prevId XOR nextId
        Node(int v) { value = v; both = 0; }
    }

    static class XorList {
        List<Node> pool = new ArrayList<>();
        int head = 0; // 0 == null sentinel
        int tail = 0;

        XorList() { pool.add(null); } // index 0 reserved as null

        Node node(int id) { return id == 0 ? null : pool.get(id); }

        void add(int element) {
            Node node = new Node(element);
            pool.add(node);
            int id = pool.size() - 1;
            if (head == 0) {
                head = tail = id;
            } else {
                pool.get(tail).both ^= id;  // old tail next becomes id
                node.both = tail;           // prev = old tail, next = 0
                tail = id;
            }
        }

        int get(int index) {
            int prev = 0, cur = head;
            for (int i = 0; i < index && cur != 0; i++) {
                int next = pool.get(cur).both ^ prev;
                prev = cur;
                cur = next;
            }
            if (cur == 0) throw new IndexOutOfBoundsException("index out of range");
            return pool.get(cur).value;
        }
    }

    public static void main(String[] args) {
        XorList list = new XorList();
        for (int v : new int[]{10, 20, 30, 40, 50}) list.add(v);
        System.out.println("get(0) = " + list.get(0));
        System.out.println("get(2) = " + list.get(2));
        System.out.println("get(4) = " + list.get(4));
    }
}
