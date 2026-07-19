// Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
// Modeled with a small JSON-like variant; recursion over the tree. Time/Space O(total keys).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    bool isDict;
    int val;                                   // leaf value
    vector<pair<string, Node>> children;       // ordered map for dicts
};

void flatten(const Node& n, const string& prefix, vector<pair<string, int>>& out) {
    if (!n.isDict) { out.push_back(make_pair(prefix, n.val)); return; }
    for (size_t i = 0; i < n.children.size(); i++) {
        const string& k = n.children[i].first;
        const Node& child = n.children[i].second;
        string key = prefix.empty() ? k : prefix + "." + k;
        flatten(child, key, out);
    }
}

int main() {
    // { key:3, foo:{ a:5, bar:{ baz:8 } } }
    Node leaf3; leaf3.isDict = false; leaf3.val = 3;
    Node leaf5; leaf5.isDict = false; leaf5.val = 5;
    Node leaf8; leaf8.isDict = false; leaf8.val = 8;
    Node bar;  bar.isDict = true;  bar.val = 0;  bar.children.push_back(make_pair("baz", leaf8));
    Node foo;  foo.isDict = true;  foo.val = 0;
    foo.children.push_back(make_pair("a", leaf5));
    foo.children.push_back(make_pair("bar", bar));
    Node root; root.isDict = true; root.val = 0;
    root.children.push_back(make_pair("key", leaf3));
    root.children.push_back(make_pair("foo", foo));

    vector<pair<string, int>> out;
    flatten(root, "", out);
    cout << "{\n";
    for (size_t i = 0; i < out.size(); i++)
        cout << "    \"" << out[i].first << "\": " << out[i].second
             << (i + 1 < out.size() ? "," : "") << "\n";
    cout << "}\n";
}
