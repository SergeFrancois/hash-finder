# Hash finder
It's a utility for searching for SHA-256 hashes with a specified number of 16-bit trailing zeros. It utilizes all processor cores for accelerated hash search.
**Notes:**
- The hash is calculated from an integer converted into a byte array using little-endian order.
- Trailing zeros are zeros in the most significant bytes of the hash array.
## Build
To build the utility, execute the following command:
```
cargo build --release
```
## Run
To search for hashes, execute the following commands:
```
cd target/release
hash_finder -N 4 -F 10
```
### Parameters
- Parameter N specifies the number of 16-bit zeros at the end of the hash.
- Parameter F specifies the number of hashes to be found.
### Result format
```
48057, 13292d1f063ab50e9bfef0470258e9a15de6f64f8eaf6a7a69e828781ba30000
102122, a9695177281a9be0bd18bce400f95e5713a068f764d1e6f5110afc6065210000
130436, 941c69292979c85f96dd63f0771420c254a06a613337fde1f2934ca7f3ae0000
183958, 1e64f3edcff57883d52caea108c3e809f65c93f827b6a6ab13e6a30649620000
...
```
Each line consists of two elements separated by a comma:
1) an integer for which the hash is found;
2) the hash in hexadecimal encoding.