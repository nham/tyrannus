use std::iter::Chain;

struct StringMatch<'a> {
    smatch: Option<Match<'a>>,
}

type Match<'a> = (&'a [char], &'a [char]);
type Parser<'a, I> = fn(&'a [char]) -> I;

impl<'a> Iterator<Match<'a>> for StringMatch<'a> {
    fn next(&mut self) -> Option<Match<'a>> {
        if self.smatch.is_some() {
            let this = self.smatch;
            self.smatch = None;
            this
        } else {
            self.smatch
        }
    }
}


static abc: &'static[char] = &['a', 'b', 'c'];
static de: &'static[char] = &['d', 'e'];

fn str_abc(inp: &[char]) -> StringMatch {
    if inp.starts_with(abc) {
        let matched = (abc, inp.slice_from(abc.len()));
        StringMatch { smatch: Some(matched) }
    } else {
        StringMatch { smatch: None }
    }

}

fn str_de(inp: &[char]) -> StringMatch {
    if inp.starts_with(de) {
        let matched = (de, inp.slice_from(de.len()));
        StringMatch { smatch: Some(matched) }
    } else {
        StringMatch { smatch: None }
    }
}

// assuming theres some way to get the iterator types of str_abc
// and str_de when we generate these
fn alt_abc_de(inp: &[char]) -> Chain<StringMatch, StringMatch> {
    str_abc(inp).chain(str_de(inp))
}

fn cat_abc_de(inp: &[char]) -> Concatter<StringMatch, StringMatch> {
    let mut it = str_abc(inp);
    let start = it.next();

    Concatter { orig_input: inp,
                first_iter: it,
                parser: str_de,
                curr: start,
                second_iter: None }
}

struct Concatter<'a, I, J> {
    orig_input: &'a [char],
    first_iter: I,
    parser: Parser<'a, J>,
    curr: Option<Match<'a>>, // None signals the iterator is exhausted
    second_iter: Option<J>,
}

/*
macro_rules! try_opt(
    ($e:expr) => (match $e { Some(x) => x, None => return None })
)
*/


fn split_at<T>(sl: &[T], mid: uint) -> (&[T], &[T]) {
    (sl.slice_to(mid), sl.slice_from(mid))
}

impl<'a, I: Iterator<Match<'a>>, J: Iterator<Match<'a>>> Concatter<'a, I, J> {
    // looks at current match (self.curr) from first parser and parses its 
    // remaining input, assigning the resulting stream of matches to second_iter
    fn get_second_iter(&mut self) -> bool {
        match self.curr {
            None => false,
            Some((_, rem)) => {
                self.second_iter = Some( (self.parser)(rem) );
                true
            }
        }
    }
}

impl<'a, I: Iterator<Match<'a>>, J: Iterator<Match<'a>>>
Iterator<Match<'a>> for Concatter<'a, I, J> {
    fn next(&mut self) -> Option<Match<'a>> {
        // should only happen at the beginning
        if self.second_iter.is_none() {
            if !self.get_second_iter() { return None; }
        }

        // we need an op for when the second iterator is exhausted.
        // it advances the first iterator
        if self.curr.is_none() {
            return None;
        } else {
            loop {
                match self.second_iter.get_mut_ref().next() {
                    None => {
                        // second_iter is exhausted, so (try to) get another
                        self.curr = self.first_iter.next();
                        if !self.get_second_iter() { return None; }
                    },
                    Some((_, rem)) => {
                        let n = self.orig_input.len() - rem.len();
                        return Some(split_at(self.orig_input, n));
                    }
                }
            }
        }
    }
}

fn main() {
    let v: Vec<char> = "abcdefg".chars().collect();
    let inp = v.as_slice();

    for m in str_abc(inp) {
        println!("{}", m);
    }

    println!("-------");

    for m in str_de(inp) {
        println!("{}", m);
    }

    println!("-------");

    for m in alt_abc_de(inp) {
        println!("{}", m);
    }

    println!("-------");

    for m in cat_abc_de(inp) {
        println!("{}", m);
    }
}
