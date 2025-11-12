// XOR doubly linked list simulated with a memory array; address = index.
// memory[0] is the null sentinel; both = prevIndex XOR nextIndex.
// add: O(1), get(i): O(i). Space O(n).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int value;
        int both; // prevIndex XOR nextIndex
        Node(int v) { value = v; both = 0; }
    }

    static List<Node> memory = new ArrayList<>();
    static int head = 0; // index, 0 = null
    static int tail = 0;

    static {
        memory.add(null); // reserve index 0 as null sentinel
    }

    static void add(int element) {
        Node node = new Node(element);
        memory.add(node);
        int idx = memory.size() - 1;
        if (head == 0) { head = tail = idx; return; }
        node.both = tail ^ 0;
        Node tailNode = memory.get(tail);
        tailNode.both = (tailNode.both ^ 0) ^ idx;
        tail = idx;
    }

    static int get(int index) {
        int prev = 0;
        int cur = head;
        for (int i = 0; i < index; i++) {
            int next = prev ^ memory.get(cur).both;
            prev = cur;
            cur = next;
        }
        return memory.get(cur).value;
    }

    public static void main(String[] args) {
        add(10);
        add(20);
        add(30);
        add(40);
        System.out.println(get(0));
        System.out.println(get(3));
    }
}
