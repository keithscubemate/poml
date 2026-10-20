# Haskell Syntax Decoder

Alot of the resources on hindley-milner and such are in haskell.
Haskell is a language whose users seem to seek out esoterica.
The key below should be sufficient guide to decode a type system constructed over all possible types to add two numbers.

## Types & declarations
| Syntax                    | Meaning                                                         |
|---------------------------|-----------------------------------------------------------------|
| `x :: T`                  | "x has type T" (type annotation/signature)                      |
| `a -> b`                  | function from a to b                                            |
| `a -> b -> c`             | curried 2-arg fn; really `a -> (b -> c)`                        |
| `f a`                     | type application — "f applied to type a" (e.g. `Maybe Int`)     |
| `(a, b)`                  | tuple type                                                      |
| `[a]`                     | list of `a`                                                     |
| `type X = Y`              | alias, zero new identity, fully interchangeable                 |
| `newtype X = X Y`         | new identity, zero runtime cost, exactly 1 ctor/1 field         |
| `data X = A T1 \| B T2`   | real new type, 1+ constructors ("sum type")                     |
| `∀a. T` / `forall a. T`   | T is generic over `a` — compiler picks fresh `a` per use        |
| `C a => ...`              | constraint: `a` must satisfy typeclass `C` before `...` applies |

## Functions
| Syntax                | Meaning                                                     |
|-----------------------|-------------------------------------------------------------|
| `\x -> e`             | lambda ("λx. e")                                            |
| `\x y -> e`           | multi-arg lambda, sugar for `\x -> \y -> e`                 |
| `f x`                 | application (juxtaposition = call, no parens)               |
| `f x y`               | `f` applied to `x`, result applied to `y` (currying)        |
| `` `f` ``             | backticks = call `f` infix: `` x `f` y `` = `f x y`         |
| `f $ x`               | `f (x)` — lowest-precedence apply, avoids trailing parens   |
| `f . g`               | compose: "do g, then f"                                     |
| `function`            | keyword for a 1-arg lambda that immediately pattern-matches |
| `\case ...`           | same, via LambdaCase extension                              |

## Pattern matching
| Syntax                    | Meaning                                                  |
|---------------------------|----------------------------------------------------------|
| `case e of p1 -> r1; ...` | match `e` against patterns top to bottom                 |
| `_`                       | wildcard, matches anything                               |
| `x@pat`                   | bind whole match to `x`, still destructure via `pat`     |
| `f (Just x) = ...`        | pattern match directly in a function's argument position |

## Containers / Functor family
| Op             | Meaning                                                                                                    | Mnemonic                           |
|----------------|------------------------------------------------------------------------------------------------------------|------------------------------------|
| `fmap` / `<$>` | map a plain fn over *anything* container-shaped (not just lists — `Maybe`, `Either`, functions themselves) | "map, generalized"                 |
| `<*>`          | apply a *wrapped* fn to a *wrapped* value — chain to cover multi-arg fns: `f <$> a <*> b <*> c`            | "apply, wrapped"                   |
| `>>=`          | bind: `m a -> (a -> m b) -> m b` — map + auto-flatten                                                      | Rust: `.and_then()`                |
| `>=>` / `<=<`  | compose two monadic (`a -> m b`) functions                                                                 | Kleisli composition                |
| `<|>`          | first success (Alternative)                                                                                | Rust: `.or()`                      |
| `<>`           | combine two of the same type (Semigroup)                                                                   | lists concat, strings append, etc. |

## `do`-notation (sugar for chained `>>=`)
```haskell
do
  x <- action1     -- x <- m  ==  m >>= \x -> ...
  y <- action2
  return (x + y)
```
= `action1 >>= \x -> action2 >>= \y -> return (x + y)`

| Syntax         | Meaning                                                              |
|----------------|----------------------------------------------------------------------|
| `x <- m`       | run `m`, bind result to `x` (this IS `>>=` under the hood)           |
| `let x = e`    | plain, non-monadic binding inside a `do` block                       |
| `return x`     | lift a plain value INTO the monad (NOT early-exit like other langs!) |

## Typeclasses (≈ Rust traits)
```haskell
class Shape a where
  area :: a -> Double

instance Shape Circle where
  area c = pi * r c * r c
```

| Syntax                   | Meaning                                            |
|--------------------------|----------------------------------------------------|
| `class C a where ...`    | declare an interface (methods a type must provide) |
| `instance C T where ...` | implement that interface for concrete type `T`     |

## Monad transformer stacks (e.g. `RWST`)
| Letter     | Capability                        | Rust equivalent               |
|------------|-----------------------------------|-------------------------------|
| R (Reader) | read-only ambient context         | `&Env` passed around          |
| W (Writer) | accumulate output as you go       | `Vec<T>` you `.push()` to     |
| S (State)  | threaded mutable state            | `&mut SomeStruct`             |
| T (suffix) | "stacked on top of another monad" | the inner `Result`/error type |

| Op       | Meaning                            |
|----------|------------------------------------|
| `get`    | read current State                 |
| `put x`  | overwrite State with `x`           |
| `tell x` | append `x` to Writer's accumulator |
| `ask`    | read the Reader context            |

## Misc symbols
| Symbol    | Meaning                                                                    |
|-----------|----------------------------------------------------------------------------|
| `::`      | "has type"                                                                 |
| `=>`      | typeclass constraint arrow (left of `=>` are requirements)                 |
| `->`      | function arrow                                                             |
| `@`       | as-pattern (bind + destructure) in patterns; type application in newer GHC |
| `!`       | strictness annotation (force eager eval, opt out of laziness)              |
| `' `(tick)| conventionally marks "a variant of" (`map'`), no special meaning           |
