// Day 1426: Maximum XOR of any two elements in an array.
// Approach: Binary trie of bits; for each number greedily pick opposite bit.
// Time: O(n * B), Space: O(n * B) where B = number of bits.
public class Solution {
    static class Trie {
        Trie[] child = new Trie[2];
    }

    static int findMaxXOR(int[] nums) {
        Trie root = new Trie();
        int maxXor = 0;
        final int BITS = 31;
        for (int num : nums) {
            Trie node = root;
            for (int b = BITS; b >= 0; b--) {
                int bit = (num >> b) & 1;
                if (node.child[bit] == null) node.child[bit] = new Trie();
                node = node.child[bit];
            }
            node = root;
            int cur = 0;
            for (int b = BITS; b >= 0; b--) {
                int bit = (num >> b) & 1;
                if (node.child[1 - bit] != null) {
                    cur |= (1 << b);
                    node = node.child[1 - bit];
                } else {
                    node = node.child[bit];
                }
            }
            maxXor = Math.max(maxXor, cur);
        }
        return maxXor;
    }

    public static void main(String[] args) {
        int[] nums = {3, 10, 5, 25, 2, 8};
        System.out.println(findMaxXOR(nums)); // 28
    }
}
