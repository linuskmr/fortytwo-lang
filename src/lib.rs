use std::iter::Peekable;

pub mod ast;
pub mod emitter_c;
pub mod lexer;
pub mod parser;
pub mod position_container;
pub mod position_reader;
pub mod token;

#[cfg(test)]
mod tests;

/// Advances the `iterator` while `condition` returns true.
fn iter_advance_while<Iter, Func, Elem>(iterator: &mut Peekable<Iter>, condition: Func)
where
    Iter: Iterator<Item = Elem>,
    Func: Fn(&Elem) -> bool,
{
    loop {
        match iterator.peek() {
            // Item is Some, so check the condition
            Some(item) if !condition(item) => break,
            // Always break at None
            None => break,
            _ => (),
        }
        // iterator.next() yields the same element we inspected with `match iterator.peek() { ... }`
        iterator.next();
    }
}

/// Advances the `iterator` while `condition` returns true and returns all such items.
fn iter_take_while<Iter, Func, Item>(iterator: &mut Peekable<Iter>, condition: Func) -> Vec<Item>
where
    Iter: Iterator<Item = Item>,
    Func: Fn(&Item) -> bool,
{
    let mut taken_items = Vec::new();
    loop {
        match iterator.peek() {
            // Item is Some, so check the condition
            Some(item) if !condition(item) => break,
            // Always break at None
            None => break,
            _ => (),
        }
        // iterator.next() yields the same element we inspected with `match iterator.peek() { ... }`, so it can
        // neither be None nor does not match the condition, because then we would called break in the loop and thus
        // would not be able to get here.
        taken_items.push(
            iterator
                .next()
                .expect("iterator.peek() returned different item than iterator.next()"),
        );
    }
    taken_items
}
