// Maximum XOR of any two elements using a binary trie of bits.
// Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.
public class Solution {
    static final int BITS = 32;

    static class Trie {
        Trie[] child = new Trie[2];
    }

    static int findMaxXOR(int[] nums) {
        Trie root = new Trie();
        for (int x : nums) {
            Trie node = root;
            for (int i = BITS - 1; i >= 0; i--) {
                int b = (x >> i) & 1;
                if (node.child[b] == null) node.child[b] = new Trie();
                node = node.child[b];
            }
        }
        int best = 0;
        for (int x : nums) {
            Trie node = root;
            int cur = 0;
            for (int i = BITS - 1; i >= 0; i--) {
                int b = (x >> i) & 1;
                int want = b ^ 1;
                if (node.child[want] != null) {
                    cur |= (1 << i);
                    node = node.child[want];
                } else {
                    node = node.child[b];
                }
            }
            if (cur > best) best = cur;
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {3, 10, 5, 25, 2, 8};
        System.out.println(findMaxXOR(nums));
    }
}
