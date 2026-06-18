// BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).
#include <iostream>
#include <queue>
using namespace std;

struct Node {
    char val;
    Node* left;
    Node* right;
    Node(char v) : val(v), left(nullptr), right(nullptr) {}
};

Node* deepestNode(Node* root) {
    if (!root) return nullptr;
    queue<Node*> q;
    q.push(root);
    Node* last = root;
    while (!q.empty()) {
        last = q.front(); q.pop();
        if (last->left) q.push(last->left);
        if (last->right) q.push(last->right);
    }
    return last;
}

int main() {
    Node* a = new Node('a');
    Node* b = new Node('b');
    Node* c = new Node('c');
    Node* d = new Node('d');
    a->left = b; a->right = c;
    b->left = d;
    cout << deepestNode(a)->val << '\n';
    return 0;
}
