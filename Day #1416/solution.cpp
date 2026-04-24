// Day 1416: evaluate a +/-/parenthesized expression of single digits, no eval.
// Approach: single scan with a sign stack (Basic Calculator). O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int evaluate(const string& s) {
    int result = 0, sign = 1;
    stack<int> st;       // stores sign context across parentheses
    st.push(1);
    int i = 0, n = (int)s.size();
    while (i < n) {
        char c = s[i];
        if (isdigit(c)) {
            int num = 0;
            while (i < n && isdigit(s[i])) { num = num * 10 + (s[i] - '0'); i++; }
            result += sign * st.top() * num;
            continue;
        } else if (c == '+') { sign = 1; }
        else if (c == '-') { sign = -1; }
        else if (c == '(') { st.push(sign * st.top()); sign = 1; }
        else if (c == ')') { st.pop(); }
        i++;
    }
    return result;
}

int main() {
    cout << evaluate("-1 + (2 + 3)") << "\n"; // 4
    return 0;
}
