# disambiseq

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/disambiseq/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/disambiseq/branch/main/graph/badge.svg?token=Y4Z9RJILHH)](https://codecov.io/gh/noamteyssier/disambiseq)

Creates unambiguous nucleotide mismatch libraries for
for a set of nucleotide sequences.

## Usage

I've rewritten this functionality a few times for different use cases
and put it into a standalone crate since it might be useful to others.

This is used to generate unambiguous one-off mismatch libraries for
a set of DNA sequences.

### Creating a new unambiguous set

```rust
use disambiseq::Disambiseq;

let sequences = vec![
    "ACT".to_string(),
    "AGT".to_string()
];
let dsq = Disambiseq::from_slice(&sequences);
println!("{:#?}", dsq);
```

### Visualizing the set

```text
Disambiseq {
    unambiguous: {
        "TCT": "ACT",
        "ACA": "ACT",
        "CCT": "ACT",
        "ACC": "ACT",
        "CGT": "AGT",
        "GGT": "AGT",
        "AGA": "AGT",
        "GCT": "ACT",
        "ACG": "ACT",
        "TGT": "AGT",
        "AGC": "AGT",
        "AGT": "ACT",
        "AGG": "AGT",
    },
    parents: {
        "AGT",
        "ACT",
    },
    ambiguous: {
        "ATT",
        "AAT",
    },
}
```

### Querying the Set

```rust
use disambiseq::Disambiseq;

let sequences = vec![
    "ACT".to_string(),
    "AGT".to_string()
];
let dsq = Disambiseq::from_slice(&sequences);

// retrieve a parental sequence
assert_eq!(dsq.get_parent("ACT"), Some(&"ACT".to_string()));

// retrieve a mutation sequence's parent
assert_eq!(dsq.get_parent("TCT"), Some(&"ACT".to_string()));

// exclude sequences with ambiguous parents
assert_eq!(dsq.get_parent("AAT"), None);
assert_eq!(dsq.get_parent("ATT"), None);
```
