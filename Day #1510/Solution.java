// Three stacks sharing ONE backing array of nodes (value, prevIndex) + free list.
// Three head pointers index into the single shared list. O(1) push/pop, O(n) space.
import java.util.ArrayList;

public class Solution {
    static class ThreeStacks {
        ArrayList<Integer> val = new ArrayList<>();
        ArrayList<Integer> prev = new ArrayList<>();
        int[] head = {-1, -1, -1};
        int freeHead = -1;

        int alloc(int v, int p) {
            int idx;
            if (freeHead != -1) {
                idx = freeHead;
                freeHead = prev.get(freeHead);
                val.set(idx, v); prev.set(idx, p);
            } else {
                idx = val.size();
                val.add(v); prev.add(p);
            }
            return idx;
        }

        void push(int item, int s) {
            head[s] = alloc(item, head[s]);
        }

        int pop(int s) {
            int idx = head[s];
            int v = val.get(idx);
            head[s] = prev.get(idx);
            prev.set(idx, freeHead);
            freeHead = idx;
            return v;
        }
    }

    public static void main(String[] args) {
        ThreeStacks st = new ThreeStacks();
        st.push(1, 0); st.push(2, 0);
        st.push(3, 1);
        st.push(4, 2); st.push(5, 2);
        System.out.println(st.pop(0) + " " + st.pop(2) + " " + st.pop(1) + " " + st.pop(0));
    }
}
