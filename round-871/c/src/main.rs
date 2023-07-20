use std::{convert::identity, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    while let Some(line) = lines.next() {
        let num_books: usize = line?.parse()?;

        let books: Vec<_> = (0..num_books).map(|_| {
            let line = lines.next().ok_or("expected book")??;
            let words: Vec<_> = line.split_whitespace().collect();
            let &[time, type_] = words.as_slice() else {
                Err("expected two words")?
            };
            let time = time.parse()?;
            let type_ = match type_ {
                "00" => BookType::Neither,
                "10" => BookType::Left,
                "01" => BookType::Right,
                "11" => BookType::Both,
                _ => Err("expected two bits")?,
            };
            Result::Ok(Book { time, type_ })
        }).collect::<Result<_>>()?;

        let ans = match min_time_to_learn(&books) {
            Some(x) => x as i32,
            None => -1,
        };
        println!("{ans}");
    }

    Ok(())
}

/// ans = min(min(Both), min(Left) + min(Right))
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BookType {
    Left,
    Right,
    Both,
    Neither,
}

#[derive(Debug, Clone, Copy)]
struct Book {
    time: u32,
    type_: BookType,
}

fn min_book_by_type(books: &[Book], type_: BookType) -> Option<u32> {
    books
        .iter()
        .filter(|b| b.type_ == type_)
        .map(|b| b.time)
        .min()
}

fn min_time_to_learn(books: &[Book]) -> Option<u32> {
    let both = min_book_by_type(books, BookType::Both);
    let left = min_book_by_type(books, BookType::Left);
    let right = min_book_by_type(books, BookType::Right);

    let lr = match (left, right) {
        (Some(l), Some(r)) => Some(l + r),
        _ => None,
    };

    [both, lr].into_iter().filter_map(identity).min()
}
