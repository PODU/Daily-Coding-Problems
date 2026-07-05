// Day 1771: Stack-based "basic calculator" for +,-,parentheses,single digits,unary sign.
// Single left-to-right pass with a sign/result stack. Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int evaluate(const string& s) {
    long long result = 0;
    int sign = 1;
    stack<pair<long long,int>> st; // saved (result, sign) at '('
    for (size_t i = 0; i < s.size(); i++) {
        char c = s[i];
        if (isdigit((unsigned char)c)) {
            int num = c - '0';
            result += sign * num;
            sign = 1;
        } else if (c == '+') {
            sign = 1;
        } else if (c == '-') {
            sign = -1;
        } else if (c == '(') {
            st.push({result, sign});
            result = 0;
            sign = 1;
        } else if (c == ')') {
            auto [prevRes, prevSign] = st.top(); st.pop();
            result = prevRes + prevSign * result;
            sign = 1;
        }
        // whitespace ignored
    }
    return (int)result;
}

int main() {
    cout << evaluate("-1 + (2 + 3)") << '\n';
    return 0;
}
