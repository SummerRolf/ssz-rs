use ssz_rs::prelude::*;

#[derive(PartialEq, Eq, Debug, SimpleSerialize)]
enum Bar {
    A(u32),
    B(List<bool, 32>),
}

impl Default for Bar {
    fn default() -> Self {
        Self::A(Default::default())
    }
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct Foo<const N: usize> {
    a: u32,
    b: Vector<u32, 4>,
    c: bool,
    d: Bitlist<27>,
    e: Bar,
    f: Bitvector<N>,
}

fn main() {
    let mut b = Vector::try_from(vec![3u32, 2u32, 1u32, 10u32]).unwrap();
    for i in b.iter() {
        println!("{i}");
    }
    for i in b.iter_mut() {
        *i += 1;
    }
    for i in b.iter() {
        println!("{i}");
    }

    let mut example: Foo<4> = Foo {
        a: 16u32,
        b,
        c: true,
        d: Bitlist::try_from(
            [
                true, false, false, true, true, false, true, false, true, true, false, false, true,
                true, false, true, false, true, true, false, false, true, true, false, true, false,
                true,
            ]
            .as_ref(),
        )
        .unwrap(),
        e: Bar::B(List::try_from(vec![true, true, false, false, false, true]).unwrap()),
        f: Bitvector::try_from([false, true, false, true].as_ref()).unwrap(),
    };

    println!("{example:#?}");
    let root = example.hash_tree_root().expect("can make root");
    println!("{root:#?}");

    example.b[2] = 44u32;
    example.d.pop();
    if let Bar::B(inner) = &mut example.e {
        inner.pop();
    }

    let encoding = match serialize(&example) {
        Ok(encoding) => encoding,
        Err(err) => {
            eprintln!("some error encoding: {err}");
            return
        }
    };

    let restored_example = match Foo::<4>::deserialize(&encoding) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("some error decoding: {err}");
            return
        }
    };

    println!("{restored_example:#?}");
    let root = restored_example.hash_tree_root().expect("can make root");
    println!("{root:?}");
}
