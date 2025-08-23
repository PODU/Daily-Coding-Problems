// Day 163: Evaluate Reverse Polish Notation with a stack. Push operands, on an
// operator pop two and apply. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long evalRPN(const vector<string>& tokens) {
    vector<long long> st;
    auto isOp = [](const string& t) {
        return t == "+" || t == "-" || t == "*" || t == "/";
    };
    for (const string& t : tokens) {
        if (isOp(t)) {
            long long b = st.back(); st.pop_back();
            long long a = st.back(); st.pop_back();
            if (t == "+") st.push_back(a + b);
            else if (t == "-") st.push_back(a - b);
            else if (t == "*") st.push_back(a * b);
            else st.push_back(a / b);
        } else {
            st.push_back(stoll(t));
        }
    }
    return st.back();
}

int main() {
    vector<string> tokens = {"15","7","1","1","+","-","/","3","*","2","1","1","+","+","-"};
    cout << evalRPN(tokens) << "\n"; // 5
    return 0;
}
