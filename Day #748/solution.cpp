// Most frequent subtree sum: post-order DFS computes each subtree sum, count in a map.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node *left, *right; Node(int v): val(v), left(nullptr), right(nullptr) {} };

int dfs(Node* n, unordered_map<int,int>& count){
    if(!n) return 0;
    int s = n->val + dfs(n->left, count) + dfs(n->right, count);
    count[s]++;
    return s;
}

int mostFrequentSubtreeSum(Node* root){
    unordered_map<int,int> count;
    dfs(root, count);
    int best = 0, bestCount = -1;
    for(auto& kv : count) if(kv.second > bestCount){ bestCount = kv.second; best = kv.first; }
    return best;
}

int main(){
    Node* root = new Node(5);
    root->left = new Node(2);
    root->right = new Node(-5);
    cout << mostFrequentSubtreeSum(root) << endl; // 2
    return 0;
}
