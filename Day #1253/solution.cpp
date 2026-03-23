// Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

bool isBalanced(const string& s) {
    stack<char> st;
    unordered_map<char,char> pair = {{')','('},{']','['},{'}','{'}};
    for (char c : s) {
        if (c == '(' || c == '[' || c == '{') st.push(c);
        else if (pair.count(c)) {
            if (st.empty() || st.top() != pair[c]) return false;
            st.pop();
        }
    }
    return st.empty();
}

int main() {
    cout << (isBalanced("([])[]({})") ? "true" : "false") << "\n";
    cout << (isBalanced("([)]") ? "true" : "false") << "\n";
    return 0;
}
