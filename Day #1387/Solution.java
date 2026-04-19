// BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static void inorder(Node root, List<Integer> a) {
        if (root == null) return;
        inorder(root.left, a);
        a.add(root.val);
        inorder(root.right, a);
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
        int[] p = twoSum(root, 20);
        System.out.println(p[0] + " " + p[1]);
    }
}
