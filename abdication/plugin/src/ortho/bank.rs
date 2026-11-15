//! The Orthogonal's decoded samples, shared by every voice (and every
//! instance of the plugin in the process). A set is decoded on a loader
//! thread the first time anything wants it (a slot chooses it), and freed
//! once nothing has wanted it for `EVICT_AFTER`, so only the sets in use take
//! memory (about 5 MB each).
//!
//! The audio thread never waits, allocates or frees: it reads a set through
//! a `Reader` (a read section, as in RCU), and a set that isn't decoded yet
//! reads as missing (silence) until it is, some tens of milliseconds later.
//! The loader frees a set only after taking it out of reach and waiting for
//! every read section that might still hold it to end.

use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::thread::{self, JoinHandle, Thread};
use std::time::{Duration, Instant};

use crate::ortholib::{self, SETS};

/// A set unwanted for this long is freed.
const EVICT_AFTER: Duration = Duration::from_secs(30);
/// How often the loader looks, with nothing to do.
const IDLE: Duration = Duration::from_millis(250);

/// One decoded set: its three recordings (L, M, H), at their gains.
pub struct SampleSet {
    pub recs: [Vec<f32>; 3],
}

const _: () = assert!(SETS.len() <= 64, "the wanted sets are bits of a u64");

struct Inner {
    sets: Vec<AtomicPtr<SampleSet>>,
    /// The sets wanted since the loader last looked (bit k: set k).
    wanted: AtomicU64,
    /// Which of `readers` new read sections count themselves in, and how many
    /// are open in each.
    generation: AtomicUsize,
    readers: [AtomicUsize; 2],
    stop: AtomicBool,
    loader: OnceLock<Thread>,
}

impl Inner {
    fn wake(&self) {
        if let Some(t) = self.loader.get() {
            t.unpark();
        }
    }

    /// Ask for set `k` (marks it in use; wakes the loader if it isn't decoded).
    fn want(&self, k: usize) {
        if k >= self.sets.len() {
            return;
        }
        let bit = 1u64 << k;
        let before = self.wanted.fetch_or(bit, SeqCst);
        if before & bit == 0 && self.sets[k].load(SeqCst).is_null() {
            self.wake();
        }
    }

    /// Take set `k` out of reach and free it once no read section can hold it.
    fn evict(&self, k: usize) {
        let p = self.sets[k].swap(std::ptr::null_mut(), SeqCst);
        if p.is_null() {
            return;
        }
        // New read sections count in the other generation; wait for this one's to end.
        let old = self.generation.fetch_add(1, SeqCst) & 1;
        while self.readers[old].load(SeqCst) != 0 {
            thread::sleep(Duration::from_millis(1));
        }
        drop(unsafe { Box::from_raw(p) });
    }

    fn run(&self) {
        let mut last_wanted: Vec<Option<Instant>> = vec![None; self.sets.len()];
        while !self.stop.load(SeqCst) {
            let wanted = self.wanted.swap(0, SeqCst);
            let now = Instant::now();
            for k in 0..self.sets.len() {
                if wanted & (1 << k) != 0 {
                    last_wanted[k] = Some(now);
                    if self.sets[k].load(SeqCst).is_null() {
                        let set = Box::new(SampleSet { recs: std::array::from_fn(|r| ortholib::decode(k, r)) });
                        self.sets[k].store(Box::into_raw(set), SeqCst);
                    }
                } else if last_wanted[k].is_some_and(|t| now - t > EVICT_AFTER) {
                    last_wanted[k] = None;
                    self.evict(k);
                }
            }
            thread::park_timeout(IDLE);
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        for p in &self.sets {
            let p = p.swap(std::ptr::null_mut(), SeqCst);
            if !p.is_null() {
                drop(unsafe { Box::from_raw(p) });
            }
        }
    }
}

/// The bank: its loader runs while any `Bank` handle lives (and stops and is
/// joined with the last, so it never outlives the plugin's code).
pub struct Bank {
    inner: Arc<Inner>,
    loader: Option<JoinHandle<()>>,
}

impl Drop for Bank {
    fn drop(&mut self) {
        self.inner.stop.store(true, SeqCst);
        self.inner.wake();
        if let Some(t) = self.loader.take() {
            let _ = t.join();
        }
    }
}

/// The process's bank, made (with its loader) if nothing holds one.
pub fn shared() -> Arc<Bank> {
    static SHARED: Mutex<Weak<Bank>> = Mutex::new(Weak::new());
    let mut shared = SHARED.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(bank) = shared.upgrade() {
        return bank;
    }
    let inner = Arc::new(Inner {
        sets: (0..SETS.len()).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
        wanted: AtomicU64::new(0),
        generation: AtomicUsize::new(0),
        readers: [AtomicUsize::new(0), AtomicUsize::new(0)],
        stop: AtomicBool::new(false),
        loader: OnceLock::new(),
    });
    let worker = inner.clone();
    let loader = thread::Builder::new()
        .name("abdication-ortho-loader".into())
        .spawn(move || worker.run())
        .expect("the Orthogonal's loader thread");
    let _ = inner.loader.set(loader.thread().clone());
    let bank = Arc::new(Bank { inner, loader: Some(loader) });
    *shared = Arc::downgrade(&bank);
    bank
}

impl Bank {
    /// Mark set `k` in use (decoding it if it isn't), without reading it:
    /// the engine does this for every chosen set, so a set is ready before
    /// its first note.
    pub fn want(&self, k: usize) {
        self.inner.want(k);
    }

    /// Open a read section: sets read through it stay valid until it's dropped.
    pub fn read(&self) -> Reader<'_> {
        let generation = self.inner.generation.load(SeqCst) & 1;
        self.inner.readers[generation].fetch_add(1, SeqCst);
        Reader { inner: &self.inner, generation }
    }

    /// Wait (up to `timeout`) until set `k` is decoded; for tests and the
    /// offline renders.
    pub fn wait_for(&self, k: usize, timeout: Duration) -> bool {
        let start = Instant::now();
        loop {
            self.want(k);
            if !self.inner.sets[k].load(SeqCst).is_null() {
                return true;
            }
            if start.elapsed() > timeout {
                return false;
            }
            self.inner.wake();
            thread::sleep(Duration::from_millis(2));
        }
    }
}

pub struct Reader<'a> {
    inner: &'a Inner,
    generation: usize,
}

impl Reader<'_> {
    /// Set `k`, if it's decoded (marking it wanted either way).
    pub fn get(&self, k: usize) -> Option<&SampleSet> {
        self.inner.want(k);
        let p = self.inner.sets.get(k)?.load(SeqCst);
        unsafe { p.as_ref() }
    }
}

impl Drop for Reader<'_> {
    fn drop(&mut self) {
        self.inner.readers[self.generation].fetch_sub(1, SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_load_on_demand_and_readers_hold_them() {
        let bank = shared();
        assert!(bank.wait_for(3, Duration::from_secs(10)), "set 3 never loaded");
        let reader = bank.read();
        let set = reader.get(3).expect("loaded");
        assert_eq!(set.recs[1].len(), SETS[3].recs[1].frames as usize);
        // An eviction waits for the open read section.
        let inner = bank.inner.clone();
        let evicting = thread::spawn(move || inner.evict(3));
        thread::sleep(Duration::from_millis(30));
        assert!(!evicting.is_finished(), "freed a set a reader holds");
        assert!(set.recs[1].iter().any(|v| *v != 0.0));
        drop(reader);
        evicting.join().unwrap();
        assert!(bank.read().get(3).is_none() || bank.wait_for(3, Duration::from_secs(10)));
    }
}
