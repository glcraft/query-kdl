# Query KDL - Specifications 1.0

This document describe how to query [KDL](https://kdl.dev) format using this crate. 

## General descirption

This specification is inspired from glob path, [XPath](https://www.w3.org/TR/1999/REC-xpath-19991116/) and KDL syntax to make queries simple and intuitive.

## Node selection

Each node is separated with `/` slash character, like `node1/node2/node3`


| **node type** | **description** |
|:--:|:---|
| `<name>` | Selects named nodes under the current node |
| `**` | Selects every node under the current node and its descendants. |
| `*` | Selects every node of the current node. |
| `..` | Selects parent's node (from the current node) |

## Ranges

It's possible to select a range of the current node selection by using curly brackets `{}` with a Rust-based range selection in between,
or an index to get only one node.
Ranges and indexes are zero based, which means that the first element is `0`, and ranges end are inclusive.
Ranges and indexes must be attached to any node type.

Here is the table of range format:

| **range/index** | **description** |
|:--:|:---|
| `1` | selects the second element |
| `1..` | selects from the second element to the end |
| `..1` | selects fro the beginning to the second element included (So it selects the first 2 element) |
| `1..3` | selects from the second element to the forth element |
| `..` | selects from the beginning to the end. It's meaningless as this is the default behavior. *But it exists* |

So, when querying `my-node{2}`, the expected result is the third `my-node` node of the current node.

## Entries

A node can be filtered by its entries using squared brackets `[]` and entries values inside.

Each entries is separated by one or more spaces, for example `[1 2 3]`.

There are 3 methods to specify any entry :

| **method** | **description** | **example** |
|:--:|:---|:---|
| `value` | Positional argument specifier | `"text"` |
| `index=value` | Indexed argument specifier | `1="text"` (means second argument has to be `text`) |
| `name=value` | Property specifier | `entry_name="text"` or `"entry_name"="text"` |

Like ranges, argument index are zero based, as you can see above in the example.
Any entry can be skipped with `_`. for example, `[_ 2 _]` means that 3 arguments is expected, and the second argument is `2`.

Entry values are typed like KDL is:

| **value** | **type** |
|:--:|:---|
| `34` | integer number |
| `3.14` | floating point value |
| `"text"` or `text`\* | text |
| `true` | Boolean |
| `null` | Null value |

\* When the expected value is a text but it *looks like* another value type (like integer for example), the double quoted text will always be interpreted like a text.

## Interpreter result

The output of the interpreter is KDL compatible. It returns the nodes selected in a list at the root of a new document. 

## Examples

To demonstrate the queries, let's take this KDL sample as the input document:

```kdl
layout {
  pane size=1 borderless=true {
    plugin location="zellij:tab-bar"
  }
  pane split_direction="vertical" {
    pane {
      name "helix"
      command "helix"
      args "."
      focus true
      size "66%"
    }
    pane split_direction="horizontal" {
      pane stacked=true {
        pane {
          command "yazi"
          env_vars {
            YAZI_CONFIG_HOME "~/.config/yazi/zellij-ide"
          }
        }
        pane command="lazygit" expanded=true
      }
      pane
    }
  }
  pane size=2 borderless=true {
    plugin location="zellij:status-bar"
  }
}
plugins {
  zjpane location="https://github.com/FuriouZz/zjpane/releases/download/v0.2.0/zjpane.wasm"
}
load_plugins {
  zjpane
}
```
*(This is one of my zellij custom layout)*

### Get the second node of the KDL document

**Query**: `*{1}`

**Result**:

```kdl
plugins {
  zjpane location="https://github.com/FuriouZz/zjpane/releases/download/v0.2.0/zjpane.wasm"
}
```

- Get all nodes named "pane" in the document, no matter where they are
  `**/pane`

```kdl

pane size=1 borderless=true {
  plugin location="zellij:tab-bar"
}
pane split_direction="vertical" {
  pane {
    name "helix"
    command "helix"
    args "."
    focus true
    size "66%"
  }
  pane split_direction="horizontal" {
    pane stacked=true {
      pane {
        command "yazi"
        env_vars {
          YAZI_CONFIG_HOME "~/.config/yazi/zellij-ide"
        }
      }
      pane command="lazygit" expanded=true
    }
    pane
  }
}

pane {
  name "helix"
  command "helix"
  args "."
  focus true
  size "66%"
}
pane split_direction="horizontal" {
  pane stacked=true {
    pane {
      command "yazi"
      env_vars {
        YAZI_CONFIG_HOME "~/.config/yazi/zellij-ide"
      }
    }
    pane command="lazygit" expanded=true
  }
  pane
}

pane stacked=true {
  pane {
    command "yazi"
    env_vars {
      YAZI_CONFIG_HOME "~/.config/yazi/zellij-ide"
    }
  }
  pane command="lazygit" expanded=true
}

pane {
  command "yazi"
  env_vars {
    YAZI_CONFIG_HOME "~/.config/yazi/zellij-ide"
  }
}
pane command="lazygit" expanded=true
pane size=2 borderless=true {
  plugin location="zellij:status-bar"
}
```

- Filter the "test" nodes with a property "name" that is "debug", and take the first 2
  `test[name=debug]{0..1}`
