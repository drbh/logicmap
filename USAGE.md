## Run this example

```bash
cargo run --example kitties
```

## Define matches

The first part we need to define the entity that we'll be using in out logic map. In this example we'll be making a `Cat`.

We need to define the structure we are testing and it's attributes.

We also need to define the match type and the expression for that match. In this example we'll create a `HairColorMatch` match which takes a `color` string.

The `hair_color_match` function will use the `color` passed to `HairColorMatch` and compare it with the Cats `hair_color` attribute.

The last snippet of code that starts with `impl Config<Cat>` is what ties our `Cat` and the `HairColorMatch` back into a logic map. Essentially we are wrapping our custom logic with a data type that the logic map can process

```rust
use logicmap::{Brick, Card, Config, ExpResult, Statement};
use serde::{Deserialize, Serialize};

// Here we make the entity we want to test our logicmaps against
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub name: String,
    pub hair_color: String,
}

// create match type
#[derive(Debug)]
pub struct HairColorMatch {
    pub color: String,
}

// define match function and logic results
pub fn hair_color_match(target_hair_color: String) -> Box<dyn Fn(Cat) -> ExpResult + 'static> {
    Box::new(move |input: Cat| {
        ExpResult {
            descr: format!("Hair is target color - {}", target_hair_color), // a desciption of the match
            path: vec![String::from("hair_color")], // how you navigate to the answer
            index: vec![0],                         // since its a key value and not in a list
            met_flag: input.hair_color == target_hair_color, // the boolean test and result
        }
    })
}

// connect match type to logicmap Config implementation
impl Config<Cat> for HairColorMatch {
    fn expression_function(&self) -> Box<dyn Fn(Cat) -> ExpResult + 'static> {
        hair_color_match(self.color.clone())
    }
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}
```

## Evaluating matches

Now we have a nice clean way to represent entities like a `Cat` as well as matches that were interested in like `HairColorMatch`.

We can use these structures to build as many Cats as we want 😺 and articulate a match of any color we want!

In the following example we'll build the most simple logic map possible. Its a single `card`, with a single `statement` with a single `brick`. We're just checking if the cat is the color we want.

```rust
//
fn main() {
    // first we'll define the match we want.
    // we'll use this matching in the following examples
    let matching_expression = Box::new(HairColorMatch {
        color: String::from("tabby"),
    });

    println!("{:#?}", matching_expression);

    // basic example comparing a cat to the cat hair color match
    let kitty: Cat = Cat {
        name: String::from("Jameo"),
        hair_color: String::from("black"),
    };

    let card: Card<Cat> = Card {
        statements: vec![
            // single statement
            Statement {
                title: String::from("It's the perfect cat"),
                contents: vec![
                    //  single brick
                    Brick {
                        title: String::from("cat hair color check"),
                        input: kitty.clone(), // this group match can't use any class above...
                        expcheck: matching_expression.expression_function(),
                    },
                ],
            },
        ],
    };
    let report = card.report();
    println!("{:#?}", kitty);
    println!("{:#?}", report);

    // Second example with a second cat!
    let kitty2: Cat = Cat {
        name: String::from("Biggie"),
        hair_color: String::from("tabby"),
    };

    let card2: Card<Cat> = Card {
        statements: vec![
            // single statement
            Statement {
                title: String::from("It's the perfect cat"),
                contents: vec![
                    //  single brick
                    Brick {
                        title: String::from("cat hair color check"),
                        input: kitty2.clone(), // this group match can't use any class above...
                        expcheck: matching_expression.expression_function(),
                    },
                ],
            },
        ],
    };
    let report2 = card2.report();
    println!("{:#?}", kitty2);
    println!("{:#?}", report2);
}
```

## Output

```
HairColorMatch {
    color: "tabby",
}
Cat {
    name: "Jameo",
    hair_color: "black",
}
CardResult {
    met_flag: false,
    stmts: [
        StatementResult {
            title: "It\'s the perfect cat",
            met_flag: false,
            exp: [
                ExpResult {
                    descr: "Hair is target color - tabby",
                    path: [
                        "hair_color",
                    ],
                    index: [],
                    met_flag: false,
                },
            ],
        },
    ],
}
Cat {
    name: "Biggie",
    hair_color: "tabby",
}
CardResult {
    met_flag: true,
    stmts: [
        StatementResult {
            title: "It\'s the perfect cat",
            met_flag: true,
            exp: [
                ExpResult {
                    descr: "Hair is target color - tabby",
                    path: [
                        "hair_color",
                    ],
                    index: [],
                    met_flag: true,
                },
            ],
        },
    ],
}
```
