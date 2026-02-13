## Specify the memroy space 
- u32: 32 bit unsigned integer

```rs
let x: u32 = 10;
```

### Bytes/bits
- 8 bits = 1 byte

## Decimal to Binary representation
```
 12 => 00001100 
 0 * 1 + 0 * 2 + 1 * 4 + 1 * 8 + 0 * 16 + 0 * 32 + ...

 19 => 10011
 u32 => 0000000000000000000000000010011 : 32 bits
```
- the right most bit value decides whether the number is positive or negative.

## Memory Management
- Rust is fast and safe
- There is no need of manual memory management
- ownershipe
- Storing in the RAM
- allocate and deallocate
- everything is immutable
  to mutate: use mut
  ```rs
    let mut x: u32 = 10;
  ```
- strings and vectors can grow in size

## Ownership of heap variables
 - dangling pointer concept
### Ownership rules

### Transferring the ownership



