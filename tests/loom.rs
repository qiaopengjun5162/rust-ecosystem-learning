use loom::sync::Arc;
use loom::sync::atomic::AtomicUsize;
use loom::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use loom::thread;

#[test]
#[should_panic]
fn buggy_concurrent_inc() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let this: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();
                thread::spawn(move || {
                    let curr = num.load(Acquire);
                    // This is a bug! this is an not atomic operation
                    // 这是一个竞态条件的例子：
                    // 假设 num 初始为 0
                    // 线程1: curr1 = num.load()  // curr1 = 0
                    // 线程2: curr2 = num.load()  // curr2 = 0
                    // 线程1: num.store(curr1 + 1)  // num = 1
                    // 线程2: num.store(curr2 + 1)  // num = 1（覆盖了线程1的结果）
                    // 最终 num 的值是 1，而不是 2
                    num.store(curr + 1, Release);

                    // fix
                    // num.fetch_add(1, Release);
                })
            })
            .collect();

        for th in this {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}
