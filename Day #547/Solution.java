// Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
// Time O(n*32), Space O(n*32).
public class Solution {
    static class Trie {
        Trie[] child = new Trie[2];
    }

    static void insert(Trie root, int num) {
        Trie cur = root;
        for (int b = 31; b >= 0; b--) {
            int bit = (num >> b) & 1;
            if (cur.child[bit] == null) cur.child[bit] = new Trie();
            cur = cur.child[bit];
        }
    }

    static int maxXor(int[] nums) {
        Trie root = new Trie();
        for (int x : nums) insert(root, x);
        int best = 0;
        for (int x : nums) {
            Trie cur = root;
            int curXor = 0;
            for (int b = 31; b >= 0; b--) {
                int bit = (x >> b) & 1;
                int want = bit ^ 1;
                if (cur.child[want] != null) {
                    curXor |= (1 << b);
                    cur = cur.child[want];
                } else {
                    cur = cur.child[bit];
                }
            }
            best = Math.max(best, curXor);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {3, 10, 5, 25, 2, 8};
        System.out.println(maxXor(nums));
    }
}
