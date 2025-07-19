# Multiple Dispatch

Let's completely shift the script. What if we used a multiple dispatch model of
computation?

```luau
-- multiple declarations follow "or else" semantics, from top to bottom
fibonacci(0) 0
fibonacci(1) 1
fibonacci(2) 1
fibonacci(x U32) fibonacci(x - 1) + fibonacci(x - 2)
```

We can assign types to values, then "refine" those types later on.

```luau
-- this is valid
x I32
x U16
x 42
```

Essentially, the rule is that as we go _down_, we must provide _subtypes_.

## Structs

The same rules apply to structs. Now, however, functions are pure.

```luau
Vector3 (
    x F32
    y F32
    z F32
)

len(v Vector3) sqrt(v.x^2 + v.y^2 + v.z^2)
```

Below is an example of how we can achieve polymorphism.

```luau
to_string(self _) String

Vector3 (x F32 y F32 z F32)
to_string(self Vector3) "({self.x}, {self.y}, {self.z})"
```

Essentially, we can think of `to_string` as being in an indeterminate state (a
function which accepts one argument of any type and returns a value of type
`String`). We provide a concrete implementation when the argument is of type
`Vector3`.

## Method Declaration Syntax

As syntax sugar, we could allow this:

```luau
Vector3:to_string() "({x}, {y}, {z})"
```

## Foreign Identifiers

We don't currently have a way to "extend" identifiers declared somewhere else.
Therefore, we wouldn't have a way to share a global `to_string` function.

We need to allow this kind of behavior in a _controlled_ way. Perhaps we could
do this:

```luau
Vector3 (
    x F32
    y F32
    z F32

    Vector3:to_string() "({x}, {y}, {z})"
)

Main (
    -- these are like "imports"
    to_string std.to_string
    to_string Vector3.to_string
)
```

It is quite simple to resolve conflicts at compile time in this case.
