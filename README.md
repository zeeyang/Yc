# Yc

> WIP. Tracking upstream changes in Yrs.

C binding for Yrs (https://github.com/yjs/yrs)

## Build
Build project and generate header file.
```sh
cargo build --release
```

Generate static library.
```sh
cargo lipo --release
```

## Usage

Swift binding example:

```swift
let ydoc = yrs_init()

let chars = "hello world".utf8.map { Int8($0) }
for (pos, char) in chars.enumerated() {
    yrs_insert(ydoc, UInt32(pos), char)
}
let updateBufferSize = yrs_encode_state_as_update(ydoc)
var updateBuffer = Array<UInt8>(repeating: 0, count: updateBufferSize)
yrs_read_binary_buffer(ydoc, &updateBuffer)
print(updateBuffer)

let stringBufferSize = yrs_to_string(ydoc)
var stringBuffer = Array<Int8>(repeating: 0, count: stringBufferSize)
yrs_read_text_buffer(ydoc, &stringBuffer)
let newString = String(cString: stringBuffer)
print(newString)

yrs_free(ydoc)
```
