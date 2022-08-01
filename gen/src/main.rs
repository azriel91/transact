fn main() {
    if let Some(limit) = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
    {
        if limit < 65536 {
            eprintln!("{limit} is too small, choose 65536 or greater.");
            std::process::exit(1);
        }

        println!("type, client, tx, amount");
        for i in 0..65536 {
            println!("deposit, {i}, {i}, 100000.0");
        }
        for i in 65536..limit {
            // deposit, 1, 1, 1.0
            // deposit, 2, 2, 2.0
            // deposit, 1, 3, 2.0
            // withdrawal, 1, 4, 1.5
            // dispute, 2, 4,
            // chargeback, 2, 4,
            // withdrawal, 2, 5, 3.0

            let client = i % 65536;
            if i % 31 == 0 {
                let tx = i;

                println!("deposit, {client}, {tx}, 150.0");
                println!("dispute, {client}, {tx},");
                if tx % 8 == 0 {
                    println!("chargeback, {client}, {tx},");
                } else if tx % 4 == 0 {
                    println!("resolve, {client}, {tx},");
                }
            } else if i % 2 == 0 {
                println!("deposit, {client}, {i}, 150.0");
            } else {
                println!("withdrawal, {client}, {i}, 10.0");
            }
        }
    } else {
        eprintln!(
            r#"Example usage:

```bash
gen 1000 > transactions.csv  # or
cargo run --package gen --release -- 1000000 > transactions.csv
```

Expected one argument specifying the number of transactions."#
        );

        std::process::exit(1);
    }
}
