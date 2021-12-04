use std::io;

fn get_args_into_u32vect() -> Vec<u32> {
    let mut stdin_buf = String::new();
    io::stdin().read_line(&mut stdin_buf).unwrap();
    let args: Vec<u32> = stdin_buf
        .replace("\n", "")
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    
    args
}

fn main() {
    let meta_info = get_args_into_u32vect();
    let card_cnt = *meta_info.get(0).unwrap() as usize;
    let target_num = *meta_info.get(1).unwrap();

    let cards = get_args_into_u32vect();
    let mut first_card_idx = 0;
    let mut second_card_idx = 1;
    let mut third_card_idx = 2;
    let mut approx_num = 0;
    loop {
        let sum_of_cards = cards.get(first_card_idx).unwrap()
            + cards.get(second_card_idx).unwrap()
            + cards.get(third_card_idx).unwrap();

        if sum_of_cards <= target_num && sum_of_cards > approx_num {
            approx_num = sum_of_cards;
        }

        if third_card_idx + 1 < card_cnt {
            third_card_idx += 1;
        } else if second_card_idx + 1 < third_card_idx {
            second_card_idx += 1;
            third_card_idx = second_card_idx + 1;
        } else if first_card_idx + 1 < second_card_idx {
            first_card_idx += 1;
            second_card_idx = first_card_idx + 1;
            third_card_idx = second_card_idx + 1;
        } else {
            break;
        }
    }

    println!("{}", approx_num);
}
