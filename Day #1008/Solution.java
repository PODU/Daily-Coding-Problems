// Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
// Time O(n*B), Space O(n*B), B = 31.
public class Solution {
    static final int B = 31;

    static class Node {
        Node[] child = new Node[2];
    }

    static int maximumXOR(int[] nums) {
        Node root = new Node();
        int best = 0;
        for (int x : nums) {
            Node ins = root, cur = root;
            int curXor = 0;
            for (int b = B - 1; b >= 0; b--) {
                int bit = (x >> b) & 1;
                if (ins.child[bit] == null) ins.child[bit] = new Node();
                ins = ins.child[bit];
                int want = bit ^ 1;
                if (cur.child[want] != null) { curXor |= (1 << b); cur = cur.child[want]; }
                else if (cur.child[bit] != null) cur = cur.child[bit];
            }
            best = Math.max(best, curXor);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {3, 10, 5, 25, 2, 8};
        System.out.println(maximumXOR(nums)); // 28
    }
}
