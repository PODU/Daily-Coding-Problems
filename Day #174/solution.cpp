// Demonstrates the three polymorphism types: ad-hoc (overloading), parametric (templates), subtype (virtual dispatch).
#include <bits/stdc++.h>
using namespace std;

static const double PI = 3.14159265358979323846;

// Ad-hoc polymorphism: function overloading.
int add(int a, int b) { return a + b; }
string add(const string& a, const string& b) { return a + b; }

// Parametric polymorphism: generic template.
template <typename T>
T maxOf(T a, T b) { return a > b ? a : b; }

// Subtype polymorphism: base with virtual method overridden by derived classes.
struct Shape { virtual double area() const = 0; virtual ~Shape() {} };
struct Circle : Shape { double r; Circle(double r):r(r){} double area() const override { return PI * r * r; } };
struct Square : Shape { double s; Square(double s):s(s){} double area() const override { return s * s; } };

int main() {
    cout << "Ad-hoc polymorphism: add(2,3)=" << add(2,3)
         << ", add(\"a\",\"b\")=" << add(string("a"), string("b")) << "\n";

    cout << "Parametric polymorphism: max(3,7)=" << maxOf(3,7)
         << ", max(2.5,1.5)=" << maxOf(2.5,1.5) << "\n";

    Shape* c = new Circle(1.0);
    Shape* sq = new Square(2.0);
    printf("Subtype polymorphism: Circle area=%.5f, Square area=%.1f\n", c->area(), sq->area());
    delete c; delete sq;
    return 0;
}
