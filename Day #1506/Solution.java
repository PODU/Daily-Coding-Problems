// Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
// Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).
public class Solution {
    static class Trie {
        Trie[] child = new Trie[2];
    }

    static void insertNum(Trie root, int num) {
        Trie node = root;
        for (int i = 31; i >= 0; i--) {
            int b = (num >> i) & 1;
            if (node.child[b] == null) node.child[b] = new Trie();
            node = node.child[b];
        }
    }

    static int queryBest(Trie root, int num) {
        Trie node = root;
        int best = 0;
        for (int i = 31; i >= 0; i--) {
            int b = (num >> i) & 1;
            int want = b ^ 1;
            if (node.child[want] != null) {
                best |= (1 << i);
                node = node.child[want];
            } else {
                node = node.child[b];
            }
        }
        return best;
    }

    static int findMaximumXOR(int[] nums) {
        Trie root = new Trie();
        int best = 0;
        for (int x : nums) {
            insertNum(root, x);
            best = Math.max(best, queryBest(root, x));
        }
        return best;
    }

    public static void main(String[] args) {
        int[] nums = {3, 10, 5, 25, 2, 8};
        System.out.println(findMaximumXOR(nums));
    }
}
