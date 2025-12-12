// Flatten a nested dictionary, joining keys with '.' via DFS.
// Insertion order preserved with a vector of pairs.
// Time: O(total keys), Space: O(depth) recursion.
#include <bits/stdc++.h>
using namespace std;

struct Value {
    bool isLeaf;
    int leaf;
    vector<pair<string, Value>> obj; // ordered
    Value(int v): isLeaf(true), leaf(v) {}
    Value(): isLeaf(false), leaf(0) {}
};

void flatten(const string& prefix, const Value& v, vector<pair<string,int>>& out){
    for(auto& kv : v.obj){
        string key = prefix.empty() ? kv.first : prefix + "." + kv.first;
        if(kv.second.isLeaf) out.push_back({key, kv.second.leaf});
        else flatten(key, kv.second, out);
    }
}

int main(){
    Value root;
    root.obj.push_back({"key", Value(3)});
    Value foo;
    foo.obj.push_back({"a", Value(5)});
    Value bar; bar.obj.push_back({"baz", Value(8)});
    foo.obj.push_back({"bar", bar});
    root.obj.push_back({"foo", foo});

    vector<pair<string,int>> out;
    flatten("", root, out);
    cout << "{";
    for(size_t i=0;i<out.size();i++){ cout << "'" << out[i].first << "': " << out[i].second; if(i+1<out.size()) cout << ", "; }
    cout << "}" << endl; // {'key': 3, 'foo.a': 5, 'foo.bar.baz': 8}
    return 0;
}
