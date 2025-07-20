# Mutation and Reassignment

```luau
Vector3 (
    x F32
    y F32
    z F32

    new(x F32, y F32, z F32) Vector3(x, y, z)

    Vector3:len_sq() x^2 + y^2 + z^2
    Vector3:len() len_sq():sqrt()

    Vector3:unit() {
        len = len()
        ?(len > 0.0) {
            => Vector3(x / len, y / len, z / len)
        }
        Vector3(x, y, z)
    }

    *Vector3:normalize() {
        len = len()
        ?(len > 0.0) {
            x /= len
            y /= len
            z /= len
        }
    }
)
```
