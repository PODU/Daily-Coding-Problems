// Balanced brackets via stack. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

bool isBalanced(const string& s) {
    stack<char> st;
    unordered_map<char, char> match = {{')', '('}, {']', '['}, {'}', '{'}};
    for (char c : s) {
        if (c == '(' || c == '[' || c == '{') {
            st.push(c);
        } else if (match.count(c)) {
            if (st.empty() || st.top() != match[c]) return false;
            st.pop();
        }
    }
    return st.empty();
}

int main() {
    assert(isBalanced("([)]") == false);
    assert(isBalanced("((()") == false);
    cout << (isBalanced("([])[]({})") ? "true" : "false") << "\n";
    return 0;
}
