// Day 1415: flatten a nested dictionary, namespacing keys with '.'.
// Approach: recursive DFS building prefixed keys. O(total keys) time/space.
#include <bits/stdc++.h>
using namespace std;

struct Value {
    bool isLeaf;
    int num;
    vector<pair<string,Value>> obj; // ordered
    Value(int n) : isLeaf(true), num(n) {}
    Value() : isLeaf(false), num(0) {}
};

void flatten(const string& prefix, const Value& v, vector<pair<string,int>>& out) {
    if (v.isLeaf) { out.push_back({prefix, v.num}); return; }
    for (auto& kv : v.obj) {
        string key = prefix.empty() ? kv.first : prefix + "." + kv.first;
        flatten(key, kv.second, out);
    }
}

int main() {
    Value root;
    root.obj.push_back({"key", Value(3)});
    Value foo;
    foo.obj.push_back({"a", Value(5)});
    Value bar;
    bar.obj.push_back({"baz", Value(8)});
    foo.obj.push_back({"bar", bar});
    root.obj.push_back({"foo", foo});

    vector<pair<string,int>> out;
    flatten("", root, out);
    for (auto& kv : out) cout << kv.first << ": " << kv.second << "\n";
    return 0;
}
