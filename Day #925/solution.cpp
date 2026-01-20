// Flatten nested dict, namespacing keys with '.'. Recurse; on nested object
// prepend parentKey + '.'. Insertion order kept via vector of pairs.
// Time O(total entries), Space O(depth).
#include <bits/stdc++.h>
using namespace std;

// Minimal nested value: either an int leaf or a nested ordered map.
struct Node {
    bool isDict;
    long long val;
    vector<pair<string, Node>> children;
    Node(long long v) : isDict(false), val(v) {}
    Node() : isDict(true), val(0) {}
};

void flatten(const Node& node, const string& prefix,
             vector<pair<string, long long>>& out) {
    for (auto& kv : node.children) {
        string key = prefix + kv.first;
        if (kv.second.isDict)
            flatten(kv.second, key + ".", out);
        else
            out.push_back({key, kv.second.val});
    }
}

int main() {
    // {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
    Node root;
    root.children.push_back({"key", Node(3)});
    Node foo;
    foo.children.push_back({"a", Node(5)});
    Node bar;
    bar.children.push_back({"baz", Node(8)});
    foo.children.push_back({"bar", bar});
    root.children.push_back({"foo", foo});

    vector<pair<string, long long>> out;
    flatten(root, "", out);

    cout << "{";
    for (size_t i = 0; i < out.size(); i++) {
        if (i) cout << ", ";
        cout << "\"" << out[i].first << "\": " << out[i].second;
    }
    cout << "}\n";
    return 0;
}
