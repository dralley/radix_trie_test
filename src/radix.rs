use std::fs::File;
use std::io::{BufReader, BufRead};
use radix_trie::Trie;
use radix_trie::TrieCommon;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut trie: Trie<String, bool> = Trie::new();

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|l| { trie.insert(l, false); });

    println!("{}", trie.len());
}
