// Day 116: generate() returns a root in O(1); children materialize lazily on access.
// Each node spawns children with a depth-decaying probability => finite a.s. but unbounded.
// (Demo uses a seeded Park-Miller LCG so the forced size is reproducible.)
#include <bits/stdc++.h>
using namespace std;

struct Node { int depth; Node *l=nullptr, *r=nullptr; Node(int d):depth(d){} };

long long lcg = 42;
long long nextRand(){ lcg = (lcg * 16807) % 2147483647LL; return lcg; }
int threshold(int d){ int t = 750 - 80*d; return t > 0 ? t : 0; }

// generate() is O(1): just hands back a root; the rest is built on demand.
Node* generate(){ return new Node(0); }

// force() lazily expands a node (here we expand fully to count the tree).
int force(Node* n){
    int cnt = 1;
    if(nextRand() % 1000 < threshold(n->depth)){ n->l = new Node(n->depth+1); cnt += force(n->l); }
    if(nextRand() % 1000 < threshold(n->depth)){ n->r = new Node(n->depth+1); cnt += force(n->r); }
    return cnt;
}
int main(){
    Node* root = generate();          // O(1)
    int n = force(root);              // realize lazily
    cout << "Generated a finite binary tree with " << n << " nodes\n";
    return 0;
}
