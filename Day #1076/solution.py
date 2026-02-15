# Day 1076: Curried callable class: each call returns new instance with updated total.
# __int__/__repr__ expose current value. O(1) per call, O(n) total. Space O(1).
class add_subtract:
    def __init__(self, value, sign=1):
        self._value = value
        self._sign = sign  # sign for the NEXT argument

    def __call__(self, x):
        return add_subtract(self._value + self._sign * x, -self._sign)

    def __int__(self):
        return self._value

    def __repr__(self):
        return str(self._value)

print(f"add_subtract(7) = {add_subtract(7)}")
print(f"add_subtract(1)(2)(3) = {add_subtract(1)(2)(3)}")
print(f"add_subtract(-5)(10)(3)(9) = {add_subtract(-5)(10)(3)(9)}")
