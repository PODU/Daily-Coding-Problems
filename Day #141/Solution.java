// Three stacks in one list via interleaved indexing: logical pos p of stack s -> physical p*3+s.
// push/pop O(1) amortized. Space O(total elements). Single backing list.
import java.util.*;

public class Solution {
    static class Stack {
        ArrayList<Integer> list = new ArrayList<>(); // single backing list
        int[] sizes = new int[3];                    // logical height of each stack

        void push(int item, int stackNumber) {
            int phys = sizes[stackNumber] * 3 + stackNumber;
            while (list.size() <= phys) list.add(0);
            list.set(phys, item);
            sizes[stackNumber]++;
        }
        int pop(int stackNumber) {
            if (sizes[stackNumber] == 0) throw new RuntimeException("empty stack");
            sizes[stackNumber]--;
            int phys = sizes[stackNumber] * 3 + stackNumber;
            return list.get(phys);
        }
    }

    public static void main(String[] args) {
        Stack s = new Stack();
        s.push(1, 0); s.push(2, 0);
        s.push(10, 1);
        s.push(100, 2); s.push(200, 2);
        System.out.println(s.pop(0) + " " + s.pop(2) + " " + s.pop(1) + " " + s.pop(2) + " " + s.pop(0));
        // 2 200 10 100 1
    }
}
