---
title: Hello, computer!
author: Pocky
summary: An example blog post
tags: pocky, markdown, syntax highlighting
status: test
---

# Header

This is an example blog post, with a bunch of source code in different languages
and usage of a bunch of Markdown features. We're using the reference `MarkdownPage`
implementation to generate HTML from it.

<!-- here's a comment! hopefully you can't see it! -->

<details>
<summary>August is nice</summary>
<p>Soft puppy</p>
</details>

~~I regret this text~~

| Name   | Good | Soft |
| ------ | ---- | ---- |
| August | Yes  | Yes  |
| Dot    | Yes  | Yes  |
| Mady   | Yes  | Yes  |
| Spot   | Yes  | Yes  |
| Toby   | Yes  | Yes  |

> Quote

- Bulleted list
- Bulleted list

1. Numbered list
1. Numbered list

- [x] Thing I have done
- [ ] Thing I need to do

```rust
fn main() {
	println!("hello, computer!");
}
```

```gleam
import gleam/io

fn main() {
	io.println("hello, computer!")
}
```

```zig
const std = @import("std");

fn main() {
	std.log.info("hello, computer!");
}
```

```swift
print("hello, computer!")
```

```cpp
#include <iostream>
using std::cout;

auto main() -> int {
	cout << "hello, computer!\n";
}
```

```go
import (
	"fmt"
)

func main() {
	fmt.Println("hello, computer!")
}
```

```haskell
main :: IO ()
main = do
  putstrln "hello, computer!"
```

```elm
module App exposing (main)

import Browser

main : Program () Model Update
main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }
```
