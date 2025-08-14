// Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

int minSumLevel(Node* root){
    if(!root) return -1;
    queue<Node*> q; q.push(root);
    int level = 0, best = 0; long bestSum = LONG_MAX;
    while(!q.empty()){
        int sz = q.size(); long sum = 0;
        for(int i=0;i<sz;i++){
            Node* n = q.front(); q.pop();
            sum += n->val;
            if(n->l) q.push(n->l);
            if(n->r) q.push(n->r);
        }
        if(sum < bestSum){ bestSum = sum; best = level; }
        level++;
    }
    return best;
}
int main(){
    Node* root = new Node(10);
    root->l = new Node(2); root->r = new Node(3);
    root->l->l = new Node(-5); root->l->r = new Node(1);
    cout << minSumLevel(root) << "\n"; // 2 (sum = -4)
    return 0;
}
