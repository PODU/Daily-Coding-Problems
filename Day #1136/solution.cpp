// Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *left, *right;
    Node(int v, Node* l = nullptr, Node* r = nullptr) : val(v), left(l), right(r) {}
};

int main() {
    Node* root = new Node(5,
        new Node(3, new Node(1, new Node(0), nullptr), new Node(4)),
        new Node(7, new Node(6), new Node(9, new Node(8), nullptr)));
    map<int, int> hdMap;
    queue<pair<Node*, int>> q;
    q.push({root, 0});
    while (!q.empty()) {
        Node* node = q.front().first;
        int hd = q.front().second;
        q.pop();
        hdMap[hd] = node->val;
        if (node->left) q.push(make_pair(node->left, hd - 1));
        if (node->right) q.push(make_pair(node->right, hd + 1));
    }
    string out = "[";
    bool first = true;
    for (map<int, int>::iterator it = hdMap.begin(); it != hdMap.end(); ++it) {
        if (!first) out += ", ";
        out += to_string(it->second);
        first = false;
    }
    out += "]";
    cout << out << endl;
}
