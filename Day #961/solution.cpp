// Day 961: Normalize an absolute Unix path resolving "." and "..".
// Approach: split by '/', use a stack. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string simplifyPath(const string& path) {
    vector<string> st;
    string seg;
    stringstream ss(path);
    while (getline(ss, seg, '/')) {
        if (seg.empty() || seg == ".") continue;
        if (seg == "..") { if (!st.empty()) st.pop_back(); }
        else st.push_back(seg);
    }
    string res = "/";
    for (size_t i = 0; i < st.size(); ++i) {
        res += st[i];
        if (i + 1 < st.size()) res += "/";
    }
    // preserve trailing slash if original ended with '/' and result isn't root
    if (!path.empty() && path.back() == '/' && res != "/") res += "/";
    return res;
}

int main() {
    cout << "\"" << simplifyPath("/usr/bin/../bin/./scripts/../") << "\"" << endl;
    return 0;
}
