/*

在 Rust 中，Arc 结合 RwLock 和 Mutex 都用于在多线程环境中安全地共享和修改数据。不过，这两者在数据访问控制的方式上有所不同，适用于不同的场景。下面将详细解释这两种结构的区别：
Arc<Mutex<T>>

    互斥锁（Mutex）：
        Mutex 提供了对数据的互斥访问，即任一时刻只允许一个线程修改数据。
        当一个线程通过 lock() 方法获取 Mutex 的锁时，其他试图访问该 Mutex 的线程将被阻塞，直到锁被释放。
        Mutex 适用于只有一个线程需要读写数据，或当读写操作差不多频繁时。

    使用场景：
        当数据经常被修改，或者读取和写入操作几乎同样频繁时，使用 Mutex 更合适。
        保证了在修改数据时不会有其他线程同时读取或写入，从而避免数据竞争。

Arc<RwLock<T>>

    读写锁（RwLock）：
        RwLock 允许多个线程同时读取数据，但写入数据时需要独占访问。
        当任何线程持有 RwLock 的写锁时，其他线程无法读取或写入数据。当线程持有读锁时，可以有多个线程同时读取数据，但不能写入。
        RwLock 是优化读取操作较多的情况。

    使用场景：
        当数据被读取远多于写入时，使用 RwLock 更合适。
        可以提高程序性能，因为它允许多个线程同时安全地读取数据，只在必要时才限制对数据的访问。

性能和安全考虑

    死锁风险：
        Mutex 和 RwLock 都可能导致死锁，如果不正确地管理锁的获取和释放顺序。
        RwLock 可能更容易遇到复杂的死锁情况，特别是在读取器和写入器之间可能发生的锁升级。

    开销：
        RwLock 可能在某些情况下比 Mutex 有更高的运行时开销，因为它需要更复杂的管理来允许多个读取器和单个写入器。

选择建议

选择 Mutex 还是 RwLock 应基于你的数据访问模式：

    如果写操作不频繁，而且有大量并发的读操作，选择 RwLock。
    如果读写操作差不多频繁，或者你希望简化并发控制的复杂性，选择 Mutex。

在任何情况下，使用 Arc 来包装这些锁是为了在多个线程之间共享它们的所有权和访问权限。
*/