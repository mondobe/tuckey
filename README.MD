# Tuc**key**

An evolution of [tuck5](https://github.com/mondobe.tuck5)'s basic architecture as a more traditional lexer-parser generator (removing the tags and iterative processing in favor of something closer to traditional pattern matching).

You are meant to interact with Tuckey through the `meta` system - you provide a string from which Tuckey builds a set of rules which is used to process some characters into tokens. So:

```
                        |  Text  |
Rule Text => Ruleset => |   \/   |
                        | Tokens |
```

For example, to process only positive integers without leading zeroes, you can use these rules:

```
nonzero = 1..9
digit = 0..9
posInt = nonzero:first & digit*
```

Which transforms the string `120` into these tokens:

```
{
    first:
    "1",
    {
        "2",
        "3",
        "4",
    }
}
```

From there, the token containing the first digit can be accessed with `token.get_first_child("first").unwrap()`.

## TODO: Add meta syntax guide