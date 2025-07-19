# Operator Overloading

We can imagine declaring "special" identifiers for math operations.

```luau
Vector3 (x F32, y F32, z F32)

(+)(lhs Vector3, rhs Vector3) Vector3(
    x lhs.x + rhs.x
    y lhs.y + rhs.y
    z lhs.z + rhs.z
)
```

As part of the "prelude", these operators will be defined for primitives. But we
may choose to extend them as well.

```luau
-- prelude
(+)(lhs _, rhs _)_
(+)(lhs I32, rhs I32){...}

-- your code
(+)(lhs Vector3, rhs Vector3){...}
```
