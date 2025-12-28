// Day 809: Check balanced round/curly/square brackets using a stack.
// Push opens, match closes against stack top. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

bool isBalanced(const string& s) {
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
    cout << boolalpha;
    cout << isBalanced("([])[]({})") << "\n"; // true
    cout << isBalanced("([)]") << "\n";        // false
    cout << isBalanced("((()") << "\n";        // false
    return 0;
}
