//! Dead simple Huffman encoding and decoding library
//!
//! # Example
//! ```
//! let (encoded, tree) = huffman_encode(data);
//! let decoded = huffman_decode(encoded, tree);
//! ```

#![warn(missing_docs)]

use std::collections::HashMap;

mod bitbuffer;

/// A structure to hold a Huffman tree
/// returned by [`huffman_encode`] and used by [`huffman_decode`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tree<Val: Clone> {
    #[doc(hidden)]
    Branch {
        zero: Box<Tree<Val>>,
        one : Box<Tree<Val>>,
    },
    #[doc(hidden)]
    Value(Option<Val>),
}

/// Decodes a Huffman encoded byte vector.
pub fn huffman_decode<In: Eq + std::hash::Hash + Clone + Copy>(input: Vec<u8>, tree: Option<Tree<In>>) -> Vec<In> {
    if tree.is_none() {
        return Vec::new();
    }

    let node = tree.unwrap();
    let mut buf = bitbuffer::BitReader::new(input);
    let mut res = Vec::new();

    let mut cn = &node;

    loop {
        match cn {
            Tree::Branch { zero, one } => {
                if buf.next() {
                    cn = one;
                } else {
                    cn = zero;
                }
            },
            Tree::Value(Some(v)) => {
                res.push(*v);
                cn = &node;
            },
            Tree::Value(None) => {
                return res;
            },
        }
    }
}

/// Encodes a vector to a byte vector and Huffman tree
pub fn huffman_encode<In: Eq + std::hash::Hash + Clone + Copy>(input: Vec<In>) -> (Vec<u8>, Option<Tree<In>>) {
    if input.len() == 0 {
        return (Vec::new(), None);
    }

    let freq = get_freq(&input);
    let mut freq = freq.into_iter().collect::<Vec<(In, usize)>>();
    freq.sort_by_key(|(_, a)| *a);
    let mut tree = freq.into_iter().map(|(a, b)| (Tree::Value(Some(a)), b)).collect::<Vec<(Tree<In>, usize)>>();
    tree.insert(0, (Tree::Value(None), 1));

    while tree.len() > 1 {
        let (one , b) = tree.remove(1);
        let (zero, a) = tree.remove(0);
        insert_sorted(&mut tree, (Tree::Branch { zero: Box::new(zero), one: Box::new(one) }, a + b));
    }

    let tree = tree[0].0.clone();

    fn to_lut<In: Eq + std::hash::Hash + Clone + Copy>(tree: &Tree<In>, lut: &mut HashMap<Option<In>, Vec<bool>>, prev: Vec<bool>) {
        match tree {
            Tree::Branch { zero, one } => {
                let mut z = prev.clone();
                let mut o = prev.clone();
                z.push(false);
                o.push(true);
                to_lut(zero, lut, z);
                to_lut(one, lut, o);
            },
            Tree::Value(v) => {
                lut.insert(*v, prev);
            },
        }
    }

    let mut lut = HashMap::new();
    to_lut(&tree, &mut lut, Vec::new());

    let mut wr = bitbuffer::BitWriter::new();
    for i in input.iter() {
        let i = lut.get(&Some(*i)).unwrap();
        wr.write_bits(i);
    }

    let i = lut.get(&None).unwrap();
    wr.write_bits(i);

    wr.trim_end_zeros();

    (wr.res, Some(tree))
}

fn insert_sorted<A>(a: &mut Vec<(A, usize)>, b: (A, usize)) {
    for (i, (_, w)) in a.iter().enumerate() {
        if *w > b.1 {
            a.insert(i, b);
            return;
        }
    }

    a.push(b);
}

fn get_freq<In: Eq + std::hash::Hash + Copy>(input: &Vec<In>) -> HashMap<In, usize> {
    let mut freq = HashMap::new();

    for i in input.iter() {
        freq.insert(*i, freq.get(i).unwrap_or(&0)+1);
    }

    freq
}
