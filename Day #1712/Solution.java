// In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static void inorder(Node r, List<Integer> a) {
        if (r == null) return;
        inorder(r.left, a); a.add(r.val); inorder(r.right, a);
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.right = new Node(15);
        root.right.left = new Node(11);
        root.right.right = new Node(15);
        int K = 20;
        List<Integer> a = new ArrayList<>();
        inorder(root, a);
        int i = 0, j = a.size() - 1;
        while (i < j) {
            int s = a.get(i) + a.get(j);
            if (s == K) { System.out.println(a.get(i) + " and " + a.get(j)); return; }
            else if (s < K) i++;
            else j--;
        }
        System.out.println("No pair found");
    }
}
