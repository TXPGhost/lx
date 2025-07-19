# Overriding

We could support overriding by using special syntax.

```luau
add(x I32, y I32) x + y

add!(2, 2) 5
```

As long as it's explicit, I think it could be somewhat useful. Note that the
above example could have been written this way to avoid overriding:

```luau
add(2, 2) 5
add(x I32, y I32) x + y
```

Although, maybe even this shouldn't happen implicitly. It seems like our
definitions are in conflict with one another. So maybe we _should_ require
explicit overriding.

If we want "pattern match"-like behavior, we should do so with a _single_
definition:

```luau
add {
    (2, 2) 5
    (x I32, y I32) x + y
}
```
