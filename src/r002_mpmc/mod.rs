use std::thread;

#[test]
fn mpmc() {
    let (tx, rx) = flume::bounded(0);
    let producer_num = 10;
    let consumer_num = 10;

    let mut consumers = Vec::new();
    (0..consumer_num).into_iter().for_each(|idx| {
        let rx = rx.clone();
        let c = thread::spawn(move || {
            for msg in rx.iter() {
                println!("c-{} receive: {}", idx, msg);
            }
        });
        consumers.push(c);
    });

    let mut producers = Vec::new();
    (0..producer_num).into_iter().for_each(|idx| {
        let tx = tx.clone();
        let p = thread::spawn(move || {
            for i in 0..3 {
                tx.send(format!("p-{} say hello: {}", idx, i)).unwrap();
            }
        });
        producers.push(p);
    });

    for p in producers {
        p.join().unwrap();
    }

    drop(tx);

    for c in consumers {
        c.join().unwrap();
    }
}