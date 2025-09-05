// Day 222: Normalize an absolute path (resolve . and ..).
// Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string simplifyPath(const string& path) {
    vector<string> st;
    stringstream ss(path);
    string tok;
    while (getline(ss, tok, '/')) {
        if (tok.empty() || tok == ".") continue;
        if (tok == "..") { if (!st.empty()) st.pop_back(); }
        else st.push_back(tok);
    }
    if (st.empty()) return "/";
    string res;
    for (auto& s : st) res += "/" + s;
    return res + "/"; // trailing slash (directory form)
}

int main() {
    cout << "\"" << simplifyPath("/usr/bin/../bin/./scripts/../") << "\"" << endl; // "/usr/bin/"
    return 0;
}
