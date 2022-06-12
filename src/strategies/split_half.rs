use std::io::{Error, ErrorKind};

pub fn split_half(cards: &mut Vec<i32>) -> Result<Vec<i32>, Error> {
    if cards.len() == 0 {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "cards must not be empty",
        ))
    } else {
        let mut half_1 = cards[..cards.len() / 2].to_vec();
        let half_2 = cards[cards.len() / 2..].to_vec();

        half_2.to_owned().append(&mut half_1);

        Ok(half_2)
    }
}
