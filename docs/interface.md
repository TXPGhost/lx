# Interface Idiom

This is one way we can do interfaces:

```luau
Animal (
    self ()
    self:animal_sound() String
    self:sleep() String
)
```

Then to implement it:

```luau
Bird (
    sound "chirp"

    as_animal(self Bird) Animal (
        self
        self:animal_sound() self.sound
        self:sleep() "zzz"
    )
)

main() {
    my_bird = Bird()

    print(my_bird:as_animal():animal_sound())
}
```

We could also do it this way to support "both" kinds of dispatch:

```luau
Bird (
    sound "chirp"

    Bird:animal_sound() self.sound
    Bird:sleep() "zzz"

    as_animal(self Bird) Animal (
        self
        self:animal_sound() self.animal_sound
        self:sleep() self.sleep
    )
)
```
