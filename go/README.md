# Substrait TextPlan Go Wrapper

This package provides Go bindings for the Substrait TextPlan library. It allows you to convert between Substrait TextPlan format and binary protobuf format directly from Go code.

## Prerequisites

Before using this package, you need to have the Substrait TextPlan library built and available on your system. The library should be built from the Rust code in the parent directory.

```bash
# Build the Substrait TextPlan library
cd ..
cargo build
```

## Installation

```bash
go get github.com/substrait-io/substrait-textplan/go/substrait
```

## Usage

```go
package main

import (
	"fmt"
	"log"

	"github.com/substrait-io/substrait-textplan/go/substrait"
)

func main() {
	// Create a new TextPlan instance
	tp := substrait.New()

	// Sample TextPlan
	textplan := `
	schema simple_schema {
		id i32;
		name string;
	}

	read RELATION data {
		SOURCE source;
		BASE_SCHEMA simple_schema;
	}

	ROOT {
		NAMES = [data]
	}
	`

	// Convert TextPlan to binary
	binary, err := tp.LoadFromText(textplan)
	if err != nil {
		log.Fatalf("Failed to convert TextPlan to binary: %v", err)
	}

	fmt.Printf("Converted TextPlan to binary (%d bytes)\n", len(binary))

	// Convert binary back to TextPlan
	roundTrip, err := tp.LoadFromBinary(binary)
	if err != nil {
		log.Fatalf("Failed to convert binary to TextPlan: %v", err)
	}

	fmt.Printf("Converted binary back to TextPlan:\n%s\n", roundTrip)
}
```

## Example Application

There's a simple example application in the `examples` directory that demonstrates how to convert between TextPlan and binary formats using files:

```bash
cd examples
go build simple.go

# Convert TextPlan to binary
./simple text2bin input.textplan output.bin

# Convert binary to TextPlan
./simple bin2text input.bin output.textplan
```

## License

Apache-2.0