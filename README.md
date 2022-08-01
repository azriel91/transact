# 🔀 Transact

Toy transaction library

## Development

### Processing Method

1. Transactions are streamed, and saved in blocks of at most `10_000` in a temporary directory on disk, saved with the lowest and highest IDs as the file name.
2. Disputes will find the a relevant transaction block file to look up the transaction whose amount to hold.
3. Resolves and chargebacks do the same lookup, though it is possible to optimize this by caching the disputed transactions separately, whether in memory or on disk.
4. Account values are all kept and updated in memory.


#### Invalid Transaction Handling

If a transaction cannot be correctly handled, then no change is applied to the account -- i.e. every transaction is atomic.


### Safety

Safety is largely managed through type-safety. There are 3 uses of `unreachable!` where the code path should not be hit through appropriate preconditions -- disputes are assumed to only happen to deposit transactions.

There are no uses of `unwrap` or `expect`, all errors are propagated with its own error variant. These carry enough information such that what has gone wrong should be understandable.

`cargo-audit` is run during CI to check that known vulnerable dependencies will be caught.


#### Data Model

1. Transaction types should be read only.
2. Transaction types are modelled individually.

    Even though most of the transaction types have common fields, it is more accurate to model them individually as they represent different concepts, and makes it impossible to erroneously read data that is not applicable.

    The compiler can optimize for the computer.

3. Type safety for client ID and transaction ID prevents accidental data type mismatch usage.
4. Amounts are represented as [`Decimal`], which prevents any arithmetic operations without overflow checks / saturation.
5. Error types are distinguished between whether the transactions can be honoured (`TxError`) and application failures (`Error`). Currently all transactions which result in `TxErrors` will be ignored.


### Performance

#### CPU Utilization

Since the application is IO heavy, `async` libraries have been used so that CPU will not be idle when reading from the source transactions file, or reading from / writing to transaction block files.


#### Memory

Transactions are streamed and dropped once they are processed, though they may be re-read from disk during processing.

There is some potentially avoidable memory allocation for updating `Account`s -- since the fields are all read-only, instantiating another `Account` requires cloning of disputed transaction values. An improvement is to allow the object to be fully destructured to reuse the existing memory, which does not break the data consistency guarantees for an `Account`.


### Development Sequence

1. Quality checks come first as it is easier to maintain quality incrementally, than retrofit best practices later.
2. It is useful to model input and output types before writing business logic, as this defines the contract between a service provider and consumer.
3. Automated tests were only written ([#9]) when the part that is automatically is known to be the right thing -- don't incur the cost of building the thing right, until one is sure that the right thing is built.


### Testing

* Manual testing with a simple transactions file was used while the application was being developed.
* Unit tests cover most of the transaction handling code.

A small crate to generate larger test input is included.

```bash
cargo run --package gen --release -- 1000000 > transactions.csv
```

It takes about 85 seconds to process 1,000,000 records on my 12-core machine.


[#9]: https://github.com/azriel91/transact/pull/9
[`Decimal`]:https://docs.rs/rust_decimal/latest/rust_decimal/struct.Decimal.html

