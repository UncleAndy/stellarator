# Find a Stellar vanity address using GPUs (Rust + C variant)

I originally copied this from here: https://github.com/mcf-rocks/solanity

Then I made the following changes:
1. Use Rust like main soft module;
2. Add vanity address using prefix & postfix;
3. Move patterns and keys count like cli parameters (was be hardcoded);

When it finds a match, it will log a line starting with MATCH, you will see the vanity address found and the secret key for it.

# BUILD

```
make build
```

This may take some time (2-10 minutes).

# RUN

```
cargo run --release -- <PREFIX> <SUFFIX> [count of keys needed]
```

For example:

```
cargo run --release -- IAM ANDY 1
```

But you can know that have limits for the first prefix character after "G".
For example, "GA", "GB", "GC", "GD" can found very fast. 
But other characters after "G" can take a long time or cannot be found. 
For suffix, you can use any characters.

Use "?" to match any character.
