// BST two-sum: in-order traversal -> sorted list, two-pointer scan.
// O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static void inorder(Node r, List<Integer> v) {
        if (r == null) return;
        inorder(r.left, v); v.add(r.val); inorder(r.right, v);
    }

    static int[] findPair(Node root, int k) {
        List<Integer> v = new ArrayList<>();
        inorder(root, v);
        int i = 0, j = v.size() - 1;
        while (i < j) {
            int s = v.get(i) + v.get(j);
            if (s == k) return new int[]{v.get(i), v.get(j)};
            if (s < k) i++; else j--;
        }
        return new int[]{-1, -1};
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.right = new Node(15);
        root.right.left = new Node(11);
        root.right.right = new Node(15);
        int[] p = findPair(root, 20);
        System.out.println(p[0] + " and " + p[1]);
    }
}
