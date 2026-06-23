// Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
// On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
import java.util.*;

public class Solution {
    static Deque<Integer> L = new ArrayDeque<>();
    static Deque<Integer> R = new ArrayDeque<>();
    static Deque<Integer> T = new ArrayDeque<>(); // transient buffer

    static void push(int x) { L.push(x); } // add to LEFT end

    static int pop() { // remove LEFT end
        if (L.isEmpty()) rebalanceLfromR();
        return L.pop();
    }

    static int pull() { // remove RIGHT end
        if (R.isEmpty()) rebalanceRfromL();
        return R.pop();
    }

    static void rebalanceLfromR() {
        int size = R.size();
        int half = size / 2; if (half == 0) half = 1;
        int keep = size - half;
        for (int k = 0; k < keep; k++) T.push(R.pop());
        while (!R.isEmpty()) L.push(R.pop());
        while (!T.isEmpty()) R.push(T.pop());
    }

    static void rebalanceRfromL() {
        int size = L.size();
        int half = size / 2; if (half == 0) half = 1;
        int keep = size - half;
        for (int k = 0; k < keep; k++) T.push(L.pop());
        while (!L.isEmpty()) R.push(L.pop());
        while (!T.isEmpty()) L.push(T.pop());
    }

    public static void main(String[] args) {
        push(1); push(2); push(3); // left-to-right: 3,2,1
        System.out.println(pop());   // 3
        System.out.println(pull());  // 1
        push(4);                     // now 4,2
        System.out.println(pop());   // 4
        System.out.println(pull());  // 2
    }
}
