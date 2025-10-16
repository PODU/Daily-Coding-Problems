// Day 440: Demonstrates the three kinds of polymorphism.
// ad-hoc = overloading, parametric = generics, subtype = overriding.
public class Solution {
    // Ad-hoc polymorphism: same name, type-specific implementations.
    static int addThem(int a, int b) { return a + b; }
    static String addThem(String a, String b) { return a + b; }

    // Parametric polymorphism: one generic method works for any Comparable type.
    static <T extends Comparable<T>> T maxOf(T a, T b) { return a.compareTo(b) >= 0 ? a : b; }

    // Subtype polymorphism: base reference dispatches to the override.
    abstract static class Shape { abstract double area(); }
    static class Circle extends Shape { double r; Circle(double r){this.r=r;} double area(){ return Math.PI*r*r; } }
    static class Square extends Shape { double s; Square(double s){this.s=s;} double area(){ return s*s; } }

    public static void main(String[] args) {
        System.out.println("Ad-hoc polymorphism (overloading): same name, type-specific impls.");
        System.out.println("  add(2,3) = " + addThem(2,3) + ", add(\"a\",\"b\") = " + addThem("a","b"));

        System.out.println("Parametric polymorphism (generics): one impl, any type.");
        System.out.println("  max(3,7) = " + maxOf(3,7) + ", max(\"ab\",\"zz\") = " + maxOf("ab","zz"));

        System.out.println("Subtype polymorphism (overriding): base ref, derived behavior.");
        Shape[] shapes = { new Circle(1.0), new Square(2.0) };
        for (Shape s : shapes) System.out.printf("  area = %.5f%n", s.area());
    }
}
