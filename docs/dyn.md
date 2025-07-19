# Dynamic Dispatch

We can emulate dynamic dispatch using the following pattern:

```luau
DynToString (
    self ()
    self:to_string() String
)
```

This also happens to be quite similar to a wide pointer. But we can do more
interesting things as well:

```luau
DynAdd<T> (
    lhs ()
    rhs ()
    add(_ lhs, _ rhs) T
)

main() {
    x = 10
    y = 20
    adder = DynAdd<I32>(x, y, (a, b) a + b)
    print(adder.add(adder.lhs, adder.rhs))
}

any_add<T>(obj DynAdd<T>) {
    obj.add(obj.lhs, obj.rhs)
}
```
