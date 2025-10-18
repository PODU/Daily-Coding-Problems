// Day 453: Two nodes in a BST summing to K.
// Inorder -> sorted list, then two-pointer. Time O(n), Space O(n).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static void inorder(Node n, List<Integer> out) {
        if (n == null) return;
        inorder(n.left, out);
        out.add(n.val);
        inorder(n.right, out);
    }

    static int[] twoSum(Node root, int k) {
        List<Integer> a = new ArrayList<>();
        inorder(root, a);
        int i = 0, j = a.size() - 1;
        while (i < j) {
            int s = a.get(i) + a.get(j);
            if (s == k) return new int[]{a.get(i), a.get(j)};
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
        int[] r = twoSum(root, 20);
        System.out.println(r[0] + " and " + r[1]); // 5 and 15
    }
}
