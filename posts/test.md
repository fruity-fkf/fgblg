# Code Highlighting Test

This is a test post to verify code syntax highlighting functionality.

## Rust Example

```rust
fn main() {
    let message = "Hello, World!";
    println!("{}", message);
    
    // Testing comments
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    match sum {
        15 => println!("Perfect!"),
        _ => println!("Not quite..."),
    }
}
```

## Python Example

```python
def greet(name):
    """A simple greeting function."""
    return f"Hello, {name}!"

# Testing Python syntax
numbers = [1, 2, 3, 4, 5]
squares = [n**2 for n in numbers]

if __name__ == "__main__":
    print(greet("World"))
    print(f"Squares: {squares}")
```

## JavaScript Example

```javascript
// Testing JavaScript syntax
const greet = (name) => {
    return `Hello, ${name}!`;
};

class Calculator {
    constructor() {
        this.value = 0;
    }
    
    add(n) {
        this.value += n;
        return this;
    }
}

console.log(greet("World"));
```

## HTML Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
    <style>
        .highlight {
            color: #ff0000;
        }
    </style>
</head>
<body>
    <h1>Hello World</h1>
    <p class="highlight">This is a test.</p>
</body>
</html>
```

## CSS Example

```css
/* Testing CSS syntax */
.container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
}

.highlight {
    color: #ff0000;
    font-weight: bold;
    text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
}
```

## Shell Example

```bash
#!/bin/bash

# Testing shell script syntax
echo "Hello, World!"

for i in {1..5}; do
    echo "Count: $i"
done

if [ -f "test.txt" ]; then
    cat test.txt
else
    echo "File not found"
fi
``` 