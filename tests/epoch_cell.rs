use epoch_cell::EpochCell;
use rand::prelude::*;

#[test]
fn consistency_test() {
    let mut rng = rand::thread_rng();
    let val1: u32 = rng.gen();
    let val2: u32 = loop {
        let sample = rng.gen();
        if sample != val1 {
            break sample;
        }
    };

    let cell = EpochCell::new(val1);

    {
        let pinned = cell.pinned();

        let ref_ = pinned.as_ref();
        assert_eq!(ref_, Some(&val1));

        pinned.set(val2);
        assert_eq!(ref_, Some(&val1));
    }

    {
        let pinned = cell.pinned();
        let ref_ = pinned.as_ref();
        assert_eq!(ref_, Some(&val2));
    }
}
