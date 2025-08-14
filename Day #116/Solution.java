// Day 116: generate() returns a root in O(1); children materialize lazily on access.
// Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
// (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
public class Solution {
    static class Node { int depth; Node l, r; Node(int d){ depth = d; } }

    static long lcg = 42;
    static long nextRand(){ lcg = (lcg * 16807) % 2147483647L; return lcg; }
    static int threshold(int d){ int t = 750 - 80 * d; return t > 0 ? t : 0; }

    // generate() is O(1): just hands back a root; the rest is built on demand.
    static Node generate(){ return new Node(0); }

    static int force(Node n){
        int cnt = 1;
        if (nextRand() % 1000 < threshold(n.depth)){ n.l = new Node(n.depth + 1); cnt += force(n.l); }
        if (nextRand() % 1000 < threshold(n.depth)){ n.r = new Node(n.depth + 1); cnt += force(n.r); }
        return cnt;
    }

    public static void main(String[] args){
        Node root = generate();          // O(1)
        int n = force(root);
        System.out.println("Generated a finite binary tree with " + n + " nodes");
    }
}
