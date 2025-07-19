# Inlining

We can use this technique for "imports".

```luau
-- this...
x a
y b
z c

-- ...is equivalent to this
..(
    x a
    y b
    z c
)
```

Here's an example:

```luau
-- vector3.lx

-- in here goes code we want part of the `Vector3` namespace
Vector3 (
    x F32
    y F32
    z F32

    new(x F32, y F32, z F32) Vector3(x, y, z)
)

-- in here goes code we want to be "globally" defined
(+)(lhs Vector3, rhs Vector3) Vector3.new(
    lhs.x + rhs.x
    lhs.y + rhs.y
    lhs.z + rhs.z
)
Vector3:to_string() "({x}, {y}, {z})"

-- main.lx

-- this is our "import" statement...
..vector3

-- now we get access to the overloaded functions
main() {
    my_vec = Vector3.new(1.0, 2.0, 3.0)
    triple = my_vec + my_vec + my_vec

    print(my_vec:to_string)
}
```

## No Duck Typing

This is a form of duck typing, in a sense. But not exactly. The identifiers all
get resolved at compile time. Regardless, we probably want to control how this
behavior works to a bit greater of an extent.

Perhaps we can control this to some extent. In order for us to be referring to
the same identifier when we write:

```luau
Vector3:to_string()
```

It must be the case that we have already "imported" `to_string` from the
standard library.

```luau
-- import the `to_string` module
..std.(to_string)
```

This could be part of a new syntax called "partial" inlining.

```luau
std (
    to_string ...
    print ...
)

-- import just `to_string`
..std.(to_string)

-- import everything
..std
```

Although, allowing this syntax raises some questions, since we normally require
that structs are "all or nothing" values. So perhaps it would be better to
simply write it this way:

```luau
to_string std.to_string
```

However, I'm skeptical of this, since it seems to imply that we are declaring a
_new_ `to_string` identifier.defining

## Counterargument

Maybe this form of duck typing is okay, since we can always opt-out of it if it
causes us trouble, by simply _not_ inlining the definition. We never lose any
expressiveness by doing this, it just becomes more verbose to express certain
things.

```luau
 -- error: conflicting definitions of `to_string`
 -- help: consider referring to them as `to_string_impl_X.to_string`
..to_string_impl_a
..to_string_impl_b
```

We would still need the "dyn" types to express polymorphic behavior anyway, so
it's not like it's really a problem.
