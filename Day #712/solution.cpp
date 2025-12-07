// Day 712: Balanced brackets check using a stack. Time O(n), space O(n).
#include <bits/stdc++.h>
using namespace std;

bool balanced(const string& s) {
    stack<char> st;
    unordered_map<char, char> match = {{')', '('}, {']', '['}, {'}', '{'}};
    for (char c : s) {
        if (c == '(' || c == '[' || c == '{') st.push(c);
        else if (match.count(c)) {
            if (st.empty() || st.top() != match[c]) return false;
            st.pop();
        }
    }
    return st.empty();
}

int main() {
    cout << (balanced("([])[]({})") ? "true" : "false") << endl;
    cout << (balanced("([)]") ? "true" : "false") << endl;
    cout << (balanced("((()") ? "true" : "false") << endl;
    return 0;
}
