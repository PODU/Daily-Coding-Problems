// Three stacks in one array via fixed equal regions, each with its own top. O(1) push/pop.
public class Solution {
    static class ThreeStacks {
        private final int cap;
        private final int[] arr;
        private final int[] top = new int[3];

        ThreeStacks(int perStack) {
            cap = perStack;
            arr = new int[3 * perStack];
        }
        void push(int item, int sn) {
            if (top[sn] >= cap) throw new RuntimeException("stack full");
            arr[sn * cap + top[sn]++] = item;
        }
        int pop(int sn) {
            if (top[sn] <= 0) throw new RuntimeException("stack empty");
            return arr[sn * cap + (--top[sn])];
        }
    }

    public static void main(String[] args) {
        ThreeStacks s = new ThreeStacks(3);
        s.push(1, 0); s.push(2, 0);
        s.push(10, 1); s.push(20, 1);
        s.push(100, 2);
        System.out.println("stack0 pop: " + s.pop(0));
        System.out.println("stack1 pop: " + s.pop(1));
        System.out.println("stack2 pop: " + s.pop(2));
        System.out.println("stack0 pop: " + s.pop(0));
    }
}
