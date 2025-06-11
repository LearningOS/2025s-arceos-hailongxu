
use core::fmt::Debug;
use core::hash::Hasher;
use core::hash::Hash;

extern crate alloc;
pub use alloc::collections::*;

use alloc::vec::Vec;
use alloc::alloc::Global;

pub struct HashMap<K,V> {
    builder: MyBuildHasher,
    vec: Vec<Option<(K,V)>>,
    len: u64,
    log: bool,
}

impl<K,V> HashMap<K,V>
    where K: Hash
{
    pub fn new()->Self {
        use arceos_api::modules::axhal::misc::random;
        let hashid = random();
        let k1 = (hashid >> 64) as u64;
        let k2 = hashid as u64;
        Self {
            builder: MyBuildHasher { k1, k2 },
            vec: Vec::new(),
            len: 0,
            log: false,
        }
    }
}

impl<K,V> HashMap<K,V>
    where K:Clone+Hash+Debug, V:Clone+Debug
{
    pub fn insert(&mut self, k:K, v:V) {
        if self.len * 10 / 7 >= self.vec.len() as u64 {
            self.rehash();
        }
        let hashid = make_hash(&self.builder,&k) as usize;
        let mut index = hashid % self.vec.len();
        loop {
            if self.vec.get(index).is_some() {
                let entry = self.vec[index+1..]
                    .iter_mut().enumerate().find(|e|{
                        e.1.is_none()
                    });
                if let Some((i,e)) = entry {
                    *e = Some((k,v));
                    self.len += 1;
                    break;
                } else {
                    self.rehash();
                    index = hashid % self.vec.len();
                }
            } else {
                self.vec[index] = Some((k,v));
                self.len += 1;
                break;
            }
        }
    }

    fn rehash(&mut self) {
        let log = self.log;
        self.log = false;
        if self.len == 0 || self.vec.len() == 0 {
            self.vec.resize(32, None);
            self.log = log;
            return;
        }
        let vec_old = core::mem::take(&mut self.vec);
        self.len = 0;
        self.vec.resize(vec_old.len()*2,None);
        vec_old.into_iter()
            .filter_map(|e|e)
            .for_each(|(k,v)|{
            self.insert(k, v);
        });
        self.log = log;
    }

    pub fn iter(&self)-> impl Iterator<Item=&(K,V)> {
        self.vec.iter().filter_map(|e|{
            if let Some(e) = e {
                Some(e)
            } else {
                None
            }
        })
    }

}


use core::hash::BuildHasher;

#[cfg(not(feature = "nightly"))]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    val.hash(&mut state);
    state.finish()
}

#[cfg(feature = "nightly")]
#[cfg_attr(feature = "inline-more", inline)]
pub(crate) fn make_hash<Q, S>(hash_builder: &S, val: &Q) -> u64
where
    Q: Hash + ?Sized,
    S: BuildHasher,
{
    hash_builder.hash_one(val)
}


struct MyBuildHasher {
    k1: u64,
    k2: u64,
}

use core::hash::SipHasher;
impl BuildHasher for MyBuildHasher {
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        core::hash::SipHasher::new_with_keys(self.k1, self.k2)
    }
}