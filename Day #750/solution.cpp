// generate() returns a root in O(1); children are materialized lazily on first access.
// Each child exists with probability p<0.5, so the tree is finite (a.s.) yet unbounded.
// generate(): O(1). Traversal materializes nodes on demand.
#include <bits/stdc++.h>
using namespace std;

const double P = 0.45;

struct LazyNode {
    int value;
    mt19937* rng;
    LazyNode* _left = nullptr; bool lSet = false;
    LazyNode* _right = nullptr; bool rSet = false;
    LazyNode(int v, mt19937* g): value(v), rng(g) {}

    LazyNode* left(){
        if(!lSet){
            lSet = true;
            double u = (*rng)() / (double)mt19937::max();
            _left = (u < P) ? new LazyNode(0, rng) : nullptr;
        }
        return _left;
    }
    LazyNode* right(){
        if(!rSet){
            rSet = true;
            double u = (*rng)() / (double)mt19937::max();
            _right = (u < P) ? new LazyNode(0, rng) : nullptr;
        }
        return _right;
    }
};

LazyNode* generate(mt19937* rng){ return new LazyNode(0, rng); } // O(1)

int treeSize(LazyNode* n){
    if(!n) return 0;
    return 1 + treeSize(n->left()) + treeSize(n->right());
}

int main(){
    mt19937 rng(42);
    LazyNode* root = generate(&rng);          // O(1)
    cout << "Generated finite tree size: " << treeSize(root) << endl;
    return 0;
}
