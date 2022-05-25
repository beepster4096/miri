use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread::spawn;

#[derive(Copy, Clone)]
struct EvilSend<T>(pub T);

unsafe impl<T> Send for EvilSend<T> {}
unsafe impl<T> Sync for EvilSend<T> {}

pub fn main() {
    let mut a = AtomicUsize::new(0);
    let b = &mut a as *mut AtomicUsize;
    let c = EvilSend(b);
    unsafe {
        let j1 = spawn(move || {
            let atomic_ref = &mut *c.0;
            atomic_ref.store(32, Ordering::SeqCst)
        });

        let j2 = spawn(move || {
            let atomic_ref = &mut *c.0;
            *atomic_ref.get_mut() //~ ERROR Data race detected between Read on thread `<unnamed>` and Atomic Store on thread `<unnamed>`
        });

        j1.join().unwrap();
        j2.join().unwrap();
    }
}
