// Flatten nested dictionary, namespacing keys with '.'. Recursive DFS preserving insertion order.
// Time O(total keys), Space O(total keys).
#include <bits/stdc++.h>
using namespace std;

// Minimal ordered nested structure: value is either an int leaf or a nested map.
struct Node {
    bool isLeaf;
    int val;
    vector<pair<string, Node>> children; // ordered
    Node(int v) : isLeaf(true), val(v) {}
    Node() : isLeaf(false), val(0) {}
};

void flatten(const Node& node, const string& prefix, vector<pair<string,int>>& out) {
    if (node.isLeaf) { out.push_back({prefix, node.val}); return; }
    for (auto& kv : node.children) {
        string key = prefix.empty() ? kv.first : prefix + "." + kv.first;
        flatten(kv.second, key, out);
    }
}

int main() {
    // {"key":3,"foo":{"a":5,"bar":{"baz":8}}}
    Node root;
    root.children.push_back({"key", Node(3)});
    Node foo;
    foo.children.push_back({"a", Node(5)});
    Node bar;
    bar.children.push_back({"baz", Node(8)});
    foo.children.push_back({"bar", bar});
    root.children.push_back({"foo", foo});

    vector<pair<string,int>> out;
    flatten(root, "", out);
    for (auto& kv : out) cout << kv.first << ": " << kv.second << "\n";
    return 0;
}
