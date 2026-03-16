# serde-xpath

XPath-based XML deserialization for Rust using serde.

This crate allows you to deserialize XML documents into Rust structs using XPath expressions to specify where each field's data should come from.

> [!WARNING]
> 🤖 **AI SLOP AHEAD**
>
> This entire crate was create by Claude. All I did was feed it the integration
> test that now resides in serde_xpath/tests/derive.rs and told it to make a
> crate that supports that format. Minimal human edits have been made since
> that point so the code is likely quite unmaintainable. I just needed a small
> crate to make an assignment at work easier and figured this was the easiest
> way ¯\\_(ツ)_/¯

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
serde_xpath = { path = "serde_xpath" }
```

## Usage

### Basic Example

```rust
use serde_xpath::Deserialize;

#[derive(Deserialize, Debug)]
#[xpath("/catalog/book")]
struct Book {
    #[xpath("/@id")]
    id: String,

    #[xpath("/title", serde_xpath::Text)]
    title: String,

    #[xpath("/author/first", serde_xpath::Text)]
    author: String,
}

fn main() {
    let xml = r#"
        <catalog>
            <book id="bk101">
                <title>The Rust Programming Language</title>
                <author>
                  <first>Steve</first>
                  <last>Klabnik</last>
                </author>
            </book>
        </catalog>
    "#;

    let book: Book = serde_xpath::from_str(xml).unwrap();
    println!("{:?}", book);
}
```

### Struct-Level XPath

Use `#[xpath("/path")]` on a struct to specify the root element for deserialization. All field XPaths are relative to this root.

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/root/item")]  // All fields are relative to <item>
struct Item {
    #[xpath("/@id")]
    id: String,
}
```

### Field Attributes

#### Extracting Attribute Values

Use `/@attribute_name` to extract XML attribute values:

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/person")]
struct Person {
    #[xpath("/@id")]       // Gets <person id="...">
    id: String,

    #[xpath("/@name")]     // Gets <person name="...">
    name: String,
}
```

#### Extracting Text Content

Use `serde_xpath::Text` as the second argument to extract the text content of an element:

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/book")]
struct Book {
    #[xpath("/title", serde_xpath::Text)]  // Gets text inside <title>...</title>
    title: String,
}
```

#### Nested Structs

Fields can be other structs that also derive `serde_xpath::Deserialize`:

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/order")]
struct Order {
    #[xpath("/customer")]
    customer: Customer,
}

#[derive(serde_xpath::Deserialize)]
struct Customer {
    #[xpath("/@id")]
    id: String,

    #[xpath("/@name")]
    name: String,
}
```

#### Optional Fields

Use `Option<T>` with `#[serde(default)]` for fields that may not exist:

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/item")]
struct Item {
    #[serde(default)]
    #[xpath("/optional_field", serde_xpath::Text)]
    maybe_value: Option<String>,
}
```

If the XPath doesn't match any element, `None` is returned.

#### Collections

Use `Vec<T>` with `#[serde(default)]` for repeating elements:

```rust
#[derive(serde_xpath::Deserialize)]
#[xpath("/order")]
struct Order {
    #[serde(default)]
    #[xpath("/items/item")]
    items: Vec<Item>,
}

#[derive(serde_xpath::Deserialize)]
struct Item {
    #[xpath("/@sku")]
    sku: String,

    #[xpath("/@quantity")]
    quantity: String,
}
```

All elements matching the XPath are collected into the Vec.

### Supported Types

The following primitive types are supported for field deserialization:

- `String`
- `bool`
- `i8`, `i16`, `i32`, `i64`
- `u8`, `u16`, `u32`, `u64`
- `f32`, `f64`
- `char`
- `Option<T>` (where T is a supported type)
- `Vec<T>` (where T is a struct with `#[derive(serde_xpath::Deserialize)]`)

### XPath Subset Supported

This crate implements a minimal subset of XPath 1.0:

| Pattern | Description | Example |
|---------|-------------|---------|
| `/element` | Child element | `/book` |
| `/parent/child` | Nested path | `/catalog/book` |
| `/@attribute` | Attribute value | `/@id` |
| `/path/@attr` | Attribute of nested element | `/author/@name` |

## Limitations

The following features are **NOT** currently supported:

### XPath Features Not Supported

- Predicates: `//book[@id='123']`
- Wildcards: `//*`, `//book`
- Descendant axis: `//title`
- Parent axis: `..`
- Sibling axes: `following-sibling::`, `preceding-sibling::`
- Functions: `text()`, `contains()`, `position()`, etc.
- Namespaces: `ns:element`
- Unions: `path1 | path2`

### Serde Features Not Supported

- Enums
- Tuples and tuple structs
- Maps
- Bytes/byte buffers
- Unit types
- Renaming fields via `#[serde(rename = "...")]`
- Flattening via `#[serde(flatten)]`
- Custom deserializers via `#[serde(deserialize_with = "...")]`

### Other Limitations

- Only deserialization is supported (no serialization)
- The standard `serde::Deserialize` trait implementation returns an error; you must use `serde_xpath::from_str()`
- All XPath expressions must be absolute paths starting with `/`
- No support for XML namespaces
- No streaming/incremental parsing (entire document is loaded into memory)

## Architecture

This workspace contains two crates:

- **serde_xpath**: The main runtime crate with the deserializer, XPath parser, and error types
- **serde_xpath_derive**: The procedural macro crate that generates the `Deserialize` implementation

The crate uses [roxmltree](https://crates.io/crates/roxmltree) for XML parsing, which provides a fast, read-only DOM.

## License

[Add your license here]
