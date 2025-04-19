# Query KDL - Specifications

This document describe how to query [KDL](https://kdl.dev) format using this crate. 

## General descirption

This specification is inspired from glob path, [XPath](https://www.w3.org/TR/1999/REC-xpath-19991116/) and KDL syntax to make queries simple and intuitive.

## Node selection

Each node is separated with `/` character, like `node1/node2/node3`


| **node type** | **description** |
|:--:|:---|
| `<name>` | Selects named nodes from the current node |
| `**` | Selects every node under the current node. |
| `*` | Selects each node of the current node. |
| `..` | Selects parent's node (from the current node) |

## Ranges

It's possible to select a range of node by using curly brackets `{}` with a Rust-based range selection in between, or an index to get only one node.
Ranges and indexes are zero based indexed, which means that the first element is 0. Ranges and indexes must be attached to a node type.
Here is the table of range possible

| **range/index** | **description** |
|:--:|:---|
| `1` | selects the second element |
| `1..` | selects from the second element to the end |
| `..1` | selects fro the beggining to the second element included (So it selects the first 2 element) |
| `1..3` | selects from the second element to the forth element |
| `..` | selects from the beginning to the end. It's meanless as this is the default behavior. *But it exists* |

## Entries

A node can be filtered by its entries using squared brackets `[]` and entries values inside.
There are 3 methods to specify any entry :

| **method** | **description** | **example** |
|:--:|:---|:---|
| `value` | Positional argument specifier | `"text"` |
| `index=value` | Indexed argument specifier | `1="text"` (means second argument has to be `text`) |
| `name=value` | Property specifier | `entry_name="text"` or `"entry_name"="text"` |

Entry values can be skipped with `_`. for example, `[_ 2 _]` means that 3 arguments is expected, and the second argument is `2`.
Entry values are type based like KDL :

| **value** | **type** |
|:--:|:---|
| `34` | integer number |
| `3.14` | floating point value |
| `"text"` | text |
| `true` | Boolean |
| `null` | Null value |

A text can be represented without double quotes if it doesn't "look like" another type and without white spaces.

Each entries is separated by spaces.

## Examples

Selects all nodes in the document that has the second
