// Merge two binary trees by summing overlapping nodes; recurse and reuse the
// non-null subtree when only one side exists. Time O(n), Space O(h).
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v) : val(v), left(nullptr), right(nullptr) {}
};

Node* merge(Node* a, Node* b) {
    if (!a) return b;
    if (!b) return a;
    Node* n = new Node(a->val + b->val);
    n->left = merge(a->left, b->left);
    n->right = merge(a->right, b->right);
    return n;
}

int main() {
    // Tree1
    Node* t1 = new Node(1);
    t1->left = new Node(3);
    t1->right = new Node(2);
    t1->left->left = new Node(5);
    // Tree2
    Node* t2 = new Node(2);
    t2->left = new Node(1);
    t2->right = new Node(3);
    t2->left->right = new Node(4);
    t2->right->right = new Node(7);

    Node* m = merge(t1, t2);

    // Level-order with nulls, trimming trailing nulls
    vector<string> out;
    queue<Node*> q;
    q.push(m);
    while (!q.empty()) {
        Node* cur = q.front(); q.pop();
        if (cur) {
            out.push_back(to_string(cur->val));
            q.push(cur->left);
            q.push(cur->right);
        } else {
            out.push_back("null");
        }
    }
    while (!out.empty() && out.back() == "null") out.pop_back();

    for (size_t i = 0; i < out.size(); ++i) {
        if (i) cout << ' ';
        cout << out[i];
    }
    cout << '\n';
    return 0;
}
