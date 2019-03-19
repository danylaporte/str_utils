[![Build Status](https://travis-ci.org/danylaporte/str_utils.svg?branch=master)](https://travis-ci.org/danylaporte/str_utils)

[![codecov](https://codecov.io/gh/danylaporte/str_utils/branch/master/graph/badge.svg)](https://codecov.io/gh/danylaporte/str_utils)



Multiple traits and utility functions around string and chars.

## Documentation
[API Documentation](https://danylaporte.github.io/str_utils/str_utils)

## Example

```rust
use str_utils::cmp::EqExt;

fn main() {
    // compare accent insensitive
    assert!("Café".eq_ai("Cafe"));
}
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) or the MIT license
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT), at your
option. This file may not be copied, modified, or distributed
except according to those terms.