// generate() returns a root in O(1); children are materialized lazily on first access.
// Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
// generate(): O(1). Traversal materializes nodes on demand.
import java.util.*;

public class Solution {
    static final double P = 0.45;

    static class LazyNode {
        int value;
        Random rng;
        LazyNode left; boolean lSet = false;
        LazyNode right; boolean rSet = false;
        LazyNode(int v, Random g){ value = v; rng = g; }

        LazyNode left(){
            if(!lSet){ lSet = true; left = rng.nextDouble() < P ? new LazyNode(0, rng) : null; }
            return left;
        }
        LazyNode right(){
            if(!rSet){ rSet = true; right = rng.nextDouble() < P ? new LazyNode(0, rng) : null; }
            return right;
        }
    }

    static LazyNode generate(Random rng){ return new LazyNode(0, rng); } // O(1)

    static int treeSize(LazyNode n){
        if(n == null) return 0;
        return 1 + treeSize(n.left()) + treeSize(n.right());
    }

    public static void main(String[] args){
        Random rng = new Random(42);
        LazyNode root = generate(rng);            // O(1)
        System.out.println("Generated finite tree size: " + treeSize(root));
    }
}
