use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallvec::{smallvec, SmallVec};
use std::iter::FromIterator;

const BENCHMARK_LOOPS: i32 = 2 * 1024;

trait Vector<T>: Extend<T> + Clone {
  fn new() -> Self;
  fn push(&mut self, val: T);
  fn pop(&mut self) -> Option<T>;
  fn remove(&mut self, p: usize) -> T;
  fn insert(&mut self, n: usize, val: T);
  fn from_elem(val: T, n: usize) -> Self;
  fn from_slice(val: &[T]) -> Self;
  fn extend_from_slice(&mut self, other: &[T]);
}

impl<A: smallvec::Array> Vector<A::Item> for SmallVec<A>
where
  <A as smallvec::Array>::Item: Copy,
{
  fn new() -> Self {
    Self::new()
  }

  fn push(&mut self, val: <A as smallvec::Array>::Item) {
    self.push(val)
  }

  fn pop(&mut self) -> Option<<A as smallvec::Array>::Item> {
    self.pop()
  }

  fn remove(&mut self, p: usize) -> <A as smallvec::Array>::Item {
    self.remove(p)
  }

  fn insert(&mut self, n: usize, val: <A as smallvec::Array>::Item) {
    self.insert(n, val)
  }

  fn from_elem(val: <A as smallvec::Array>::Item, n: usize) -> Self {
    smallvec![val; n]
  }

  fn from_slice(val: &[<A as smallvec::Array>::Item]) -> Self {
    SmallVec::from_slice(val)
  }

  fn extend_from_slice(&mut self, other: &[<A as smallvec::Array>::Item]) {
    self.extend_from_slice(other)
  }
}

macro_rules! gen_default {
  ($typ:ty, $b:expr) => {
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        let vec: $typ = Default::default();
        black_box(&vec);
      }
    });
  };
}

macro_rules! gen_clone {
  ($typ:ty, $num:expr, $b:expr) => {{
    let outer = <$typ>::from_elem(0, $num);
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        black_box(&outer);
        let vec = outer.clone();
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_push {
  ($typ:ty, $num:expr, $b:expr) => {{
    let mut vec = <$typ>::new();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        vec.clear();
        black_box(&vec);
        for x in (0..($num as usize)).map(|x| x as _) {
          vec.push(x);
        }
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_insert {
  ($typ:ty, $num:expr, $b:expr) => {{
    let mut vec = <$typ>::new();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        vec.clear();
        black_box(&vec);
        // Add one element, with each iteration we insert one before the end.
        // This means that we benchmark the insertion operation and not the
        // time it takes to `ptr::copy` the data.
        vec.push(0);
        for x in 0..($num as usize) {
          vec.insert(x, x as _);
        }
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_remove {
  ($typ:ty, $num:expr, $b:expr) => {{
    let outer = <$typ>::from_elem(0, $num);
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        let mut vec = outer.clone();
        black_box(&vec);
        for x in (0..$num - 1).rev() {
          vec.remove(x);
        }
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_extend {
  ($typ:ty, $num:expr, $b:expr) => {{
    let mut vec = <$typ>::new();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        vec.clear();
        black_box(&vec);
        vec.extend((0..($num as usize)).map(|x| x as _));
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_from_iter {
  ($typ:ty, $num:expr, $b:expr) => {
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        let vec = <$typ>::from_iter((0..($num as usize)).map(|x| x as _));
        black_box(&vec);
      }
    });
  };
}

macro_rules! gen_from_slice {
  ($typ:ty, $num:expr, $b:expr) => {{
    let v: Vec<_> = (0..($num as usize)).map(|x| x as _).collect();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        let vec = <$typ>::from_slice(&v);
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_extend_from_slice {
  ($typ:ty, $num:expr, $b:expr) => {{
    let v: Vec<_> = (0..($num as usize)).map(|x| x as _).collect();
    let mut vec = <$typ>::new();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        vec.clear();
        black_box(&vec);
        vec.extend_from_slice(&v);
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_pushpop {
  ($typ:ty, $num:expr, $b:expr) => {{
    let mut vec = <$typ>::new();
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        vec.clear();
        black_box(&vec);
        for x in 0..($num as usize) {
          vec.push(x as _);
          vec.pop();
        }
        black_box(&vec);
      }
    });
  }};
}

macro_rules! gen_from_elem {
  ($typ:ty, $num:expr, $b:expr) => {
    $b.iter(|| {
      for _ in 0..BENCHMARK_LOOPS {
        let vec = <$typ>::from_elem(42, $num);
        black_box(&vec);
      }
    });
  };
}

macro_rules! make_benches {
  ($name:ident, $typ:ident<[$elem:ty; $len:expr]>) => {
    fn $name(c: &mut Criterion) {
      type Arr = $typ<[$elem; $len]>;
      let name = concat!(
        stringify!($typ),
        "<[",
        stringify!($elem),
        ";",
        stringify!($len),
        "]>"
      );

      let mut g = c.benchmark_group(name);

      g.bench_function("default", |b| gen_default!(Arr, b));

      g.bench_function("clone", |b| gen_clone!(Arr, $len * 2, b));
      g.bench_function("clone_small", |b| gen_clone!(Arr, $len, b));

      g.bench_function("push", |b| gen_push!(Arr, $len * 2, b));
      g.bench_function("push_small", |b| gen_push!(Arr, $len, b));

      g.bench_function("insert", |b| gen_insert!(Arr, $len * 2, b));
      g.bench_function("insert_small", |b| gen_insert!(Arr, $len, b));

      g.bench_function("remove", |b| gen_remove!(Arr, $len * 2, b));
      g.bench_function("remove_small", |b| gen_remove!(Arr, $len, b));

      g.bench_function("extend", |b| gen_extend!(Arr, $len * 2, b));
      g.bench_function("extend_small", |b| gen_extend!(Arr, $len, b));

      g.bench_function("from_iter", |b| gen_from_iter!(Arr, $len * 2, b));
      g.bench_function("from_iter_small", |b| gen_from_iter!(Arr, $len, b));

      g.bench_function("from_slice", |b| gen_from_slice!(Arr, $len * 2, b));
      g.bench_function("from_slice_small", |b| gen_from_slice!(Arr, $len, b));

      g.bench_function("extend_from_slice", |b| {
        gen_extend_from_slice!(Arr, $len * 2, b)
      });
      g.bench_function("extend_from_slice_small", |b| {
        gen_extend_from_slice!(Arr, $len, b)
      });

      g.bench_function("from_elem", |b| gen_from_elem!(Arr, $len * 2, b));
      g.bench_function("from_elem_small", |b| gen_from_elem!(Arr, $len, b));

      g.bench_function("pushpop", |b| gen_pushpop!(Arr, $len * 2, b));
      g.bench_function("pushpop_small", |b| gen_pushpop!(Arr, $len, b));
    }
  };
}

make_benches! { smallvec_u8_16, SmallVec<[u8; 16]> }
make_benches! { smallvec_u8_32, SmallVec<[u8; 32]> }
make_benches! { smallvec_u8_64, SmallVec<[u8; 64]> }
make_benches! { smallvec_u8_128, SmallVec<[u8; 128]> }
make_benches! { smallvec_u8_256, SmallVec<[u8; 256]> }
make_benches! { smallvec_u64_2, SmallVec<[u64; 2]> }
make_benches! { smallvec_u64_4, SmallVec<[u64; 4]> }
make_benches! { smallvec_u64_8, SmallVec<[u64; 8]> }
make_benches! { smallvec_u64_16, SmallVec<[u64; 16]> }
make_benches! { smallvec_u64_32, SmallVec<[u64; 32]> }

criterion_group!(
  benches,
  smallvec_u8_16,
  smallvec_u8_32,
  smallvec_u8_64,
  smallvec_u8_128,
  smallvec_u8_256,
  smallvec_u64_2,
  smallvec_u64_4,
  smallvec_u64_8,
  smallvec_u64_16,
  smallvec_u64_32,
);

criterion_main!(benches);
