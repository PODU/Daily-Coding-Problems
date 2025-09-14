// Day 274: Evaluate string of (), single digits, +/- without eval.
// Stack-based sign tracking. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int evaluate(const string& s) {
    int result = 0, sign = 1, i = 0, n = (int)s.size();
    stack<int> st; // stores (result, sign) pairs by pushing result then sign
    while (i < n) {
        char c = s[i];
        if (isdigit((unsigned char)c)) {
            result += sign * (c - '0'); // single digits per problem
        } else if (c == '+') {
            sign = 1;
        } else if (c == '-') {
            sign = -1;
        } else if (c == '(') {
            st.push(result);
            st.push(sign);
            result = 0;
            sign = 1;
        } else if (c == ')') {
            int s2 = st.top(); st.pop();
            int prev = st.top(); st.pop();
            result = prev + s2 * result;
        }
        i++;
    }
    return result;
}

int main() {
    cout << evaluate("-1 + (2 + 3)") << "\n"; // 4
    return 0;
}
