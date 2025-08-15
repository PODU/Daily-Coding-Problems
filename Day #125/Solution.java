// Day 125: Two nodes in a BST summing to K.
// Inorder traversal -> sorted values, two-pointer. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static void inorder(Node r, List<Integer> out) {
        if (r == null) return;
        inorder(r.left, out);
        out.add(r.val);
        inorder(r.right, out);
    }

    static int[] twoSum(Node root, int k) {
        List<Integer> v = new ArrayList<>();
        inorder(root, v);
        int i = 0, j = v.size() - 1;
        while (i < j) {
            int s = v.get(i) + v.get(j);
            if (s == k) return new int[]{v.get(i), v.get(j)};
            if (s < k) i++;
            else j--;
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
        System.out.println("Return the nodes " + r[0] + " and " + r[1] + ".");
    }
}
