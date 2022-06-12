use std::io::{Error, ErrorKind};

fn shuffle_logic(cards: &Vec<i32>, times: u8) -> Result<Vec<i32>, Error> {
    if times == 0 {
        Result::Ok(cards.to_vec())
    } else {
        let part_1 = &cards[..cards.len() / 2];
        let part_2 = &cards[cards.len() / 2..];

        let mut unsorted_vector: Vec<i32> = vec![];

        for (item_1, item_2) in part_1.iter().zip(part_2.iter()) {
            unsorted_vector.push(*item_1);
            unsorted_vector.push(*item_2);
        }

        if part_1.len() != part_2.len() {
            match part_1.len() > part_2.len() {
                true => unsorted_vector.push(part_1[part_1.len() - 1]),
                false => unsorted_vector.push(part_2[part_2.len() - 1]),
            };
        }

        match times == 0 {
            true => Ok(unsorted_vector),
            false => shuffle_logic(&unsorted_vector, times - 1),
        }
    }
}

pub fn shuffle(cards: &Vec<i32>, times: &u8) -> Result<Vec<i32>, Error> {
    if *times == 0 {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "times must be greater than 0",
        ))
    } else {
        shuffle_logic(cards, *times)
    }
}
