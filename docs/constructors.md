# Constructor Syntax

We can use positional arguments for function calls. Then, we can use a different
syntax for constructors.

```luau
Vector3 (
    x F32
    y F32
    z F32
)

main() {
    -- function syntax
    my_vec = Vector3(1.0, 2.0, 3.0)

    -- constructor syntax (type intersection)
    -- might have issues with nominal typing
    my_vec = Vector3&(x 1.0, y 2.0, z 3.0)

    -- constructor syntax (requires new syntax, but looks nice)
    my_vec = ..Vector3(x 1.0, y 2.0, z 3.0)
}
```

If we want to use named arguments in the _function_ syntax, we can use equal
signs. This reflects the fact that we are constructing a _value_.

```luau
main() {
    -- function syntax
    my_vec = Vector3(x=1.0, y=2.0, z=3.0)
}
```

It also means we can mix positional and named arguments. We can provide default
values by using an equal sign.

```luau
print(message String, newline Bool = false) {
    print(message)
    ?(newline) {
        print("\n")
    }
}

Person (
    name String
    children []Person = []
)

main() {
    print("Hello, ")
    print("World!", newline=true)

    my_person = Person("Alexander")
    my_person = Person("Alexander", [])
    my_person = Person("Alexander", children=[])
    my_person = Person(name="Alexander", children=[])
}
```

Positional arguments always go on the left, and are written in the order in
which the struct fields are declared. Named arguments may begin after the last
positional argument, and may be written in any arbitrary order.

```luau
-- general syntax
func(pos0, pos1, pos2 ... posN, name0=value, name1=value ... nameM=value)
```
