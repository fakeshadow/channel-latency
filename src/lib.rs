use futures::StreamExt;
use std::future::Future;

pub fn crossbeam_spsc(cycle: usize) {
    let (tx, rx) = crossbeam::queue::spsc::new(cycle + 1);

    std::thread::spawn(move || {
        for i in 0..cycle + 1 {
            let _ = tx.push(i);
        }
    });

    let _ = std::thread::spawn(move || loop {
        if let Ok(i) = rx.pop() {
            if i == cycle {
                break;
            }
        }
    })
    .join();
}

pub fn crossbeam_mpmc(cycle: usize) {
    let (tx, rx) = crossbeam::channel::bounded(cycle + 1);

    std::thread::spawn(move || {
        for i in 0..cycle + 1 {
            let _ = tx.send(i);
        }
    });

    let _ = std::thread::spawn(move || loop {
        if let Ok(i) = rx.recv() {
            if i == cycle {
                break;
            }
        }
    })
    .join();
}

pub fn std_mpsc(cycle: usize) {
    let (tx, rx) = std::sync::mpsc::sync_channel(cycle + 1);

    std::thread::spawn(move || {
        for i in 0..cycle + 1 {
            let _ = tx.send(i);
        }
    });

    let _ = std::thread::spawn(move || {
        while let Ok(i) = rx.recv() {
            if i == cycle {
                break;
            }
        }
    })
    .join();
}

pub fn sync_flume(cycle: usize) {
    let (tx, rx) = flume::bounded(cycle + 1);

    std::thread::spawn(move || {
        for i in 0..cycle + 1 {
            let _ = tx.send(i);
        }
    });

    let _ = std::thread::spawn(move || {
        while let Ok(i) = rx.recv() {
            if i == cycle {
                break;
            }
        }
    })
    .join();
}

pub fn tokio_flume(cycle: usize) {
    block_on(async move {
        let (tx, rx) = flume::bounded(cycle);

        tokio::spawn(async move {
            for i in 0..cycle + 1 {
                let _ = tx.send_async(i).await;
            }
        });

        let _ = tokio::spawn(async move {
            let mut rx = rx.into_stream();
            while let Some(i) = rx.next().await {
                if i == cycle {
                    break;
                }
            }
        })
        .await;
    })
}

pub fn tokio_mpsc(cycle: usize) {
    block_on(async move {
        let (mut tx, mut rx) = tokio::sync::mpsc::channel(cycle);

        tokio::spawn(async move {
            for i in 0..cycle + 1 {
                let _ = tx.send(i).await;
            }
        });

        let _ = tokio::spawn(async move {
            while let Some(i) = rx.recv().await {
                if i == cycle {
                    break;
                }
            }
        })
        .await;
    })
}

pub fn async_channel(cycle: usize) {
    async_std::task::block_on(async move {
        let (tx, rx) = async_channel::bounded(cycle + 1);
        async_std::task::spawn(async move {
            for i in 0..cycle + 1 {
                let _ = tx.send(i).await;
            }
        });

        let _ = async_std::task::spawn(async move {
            while let Ok(i) = rx.recv().await {
                if i == cycle {
                    break;
                }
            }
        })
        .await;
    });
}

fn block_on<F>(f: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f);
}
