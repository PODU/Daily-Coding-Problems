// Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
// pop on "..". Preserve a trailing slash if the input had one. Time O(n).
#include <bits/stdc++.h>
using namespace std;

string normalize(const string& path) {
    vector<string> stk;
    string comp;
    stringstream ss(path);
    while (getline(ss, comp, '/')) {
        if (comp == "" || comp == ".") continue;
        if (comp == "..") { if (!stk.empty()) stk.pop_back(); }
        else stk.push_back(comp);
    }
    string res = "/";
    for (size_t i = 0; i < stk.size(); ++i) res += stk[i] + (i + 1 < stk.size() ? "/" : "");
    bool trailing = path.size() > 1 && path.back() == '/';
    if (trailing && res != "/" && res.back() != '/') res += "/";
    return res;
}

int main() {
    cout << "\"" << normalize("/usr/bin/../bin/./scripts/../") << "\"" << endl;
    return 0;
}
