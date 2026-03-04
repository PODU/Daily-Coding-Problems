// Day 1152: Simplify absolute Unix path.
// Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string simplify(const string& path) {
    vector<string> st;
    stringstream ss(path);
    string part;
    while (getline(ss, part, '/')) {
        if (part.empty() || part == ".") continue;
        if (part == "..") { if (!st.empty()) st.pop_back(); }
        else st.push_back(part);
    }
    string res = "/";
    for (size_t i = 0; i < st.size(); ++i) res += st[i] + "/";
    return res; // trailing slash preserved (matches example)
}

int main() {
    cout << simplify("/usr/bin/../bin/./scripts/../") << "\n"; // /usr/bin/
    return 0;
}
