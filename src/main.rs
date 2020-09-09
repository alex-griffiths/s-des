use std::env;

// Global permutation arrays
const P10:      [u16; 10] = [3, 5, 2, 7, 4, 10, 1, 9, 8, 6];
const P8:       [u16; 8]  = [6, 3, 7, 4, 8, 5, 10, 9];
const _P4:       [u16; 4]  = [2, 4, 3, 1];
const _IP:       [u16; 8]  = [2, 6, 3, 1, 4, 8, 5, 7];
const _IP_INV:   [u16; 8]  = [4, 1, 3, 5, 7, 2, 8, 6];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        eprintln!("Usage: {} key data <OPTIONAL> -d\r\n", &args[0]);
        panic!();
    }

    let input_key = u16::from_str_radix(&args[1], 2).unwrap();
    let _input_data = u8::from_str_radix(&args[2], 2).unwrap();

    let _decrypt = args.len() == 4 && &args[3] == "-d";

    let keys = key_gen(input_key);

    println!("K1: {:#010b}", keys.0);
    println!("K2: {:#010b}", keys.1);
}


fn permute(d_in: u16, perm: &[u16], target_size: u16, in_size: u16) -> u16 {
    let mut out: u16 = 0; // perm output. Needs to be 0 so we can use bit ops.

    for i in 0..target_size {

        // Create a mask that we can then OR the value into the output.
        let mask = 1 << (in_size - perm[i as usize]);
        if (mask & d_in) != 0 {
             out |= 1 << (target_size - i - 1);
        }
    }

    out
}



fn key_gen(key: u16) -> (u16, u16) {
    let permuted_key = permute(key, &P10, 10, 10);
    println!("P10: {:#010b}", permuted_key);

    // Shift our key right by five bits will push our 5 most significant bits
    // (In this case, the left hand side of our key) to the be the 5 most
    // least significant bits, giving us the lef hand side on out own.
    //
    // By ANDing our key by 31 (1F) we are able to extract just the first 5 bits
    // of our key. 0x13 = 11111
    let lh_key = permuted_key >> 5;
    let rh_key = permuted_key & 0x1F; // Mask = 11111

    println!("LH: {:0b}", lh_key);
    println!("RH: {:0b}", rh_key);

    // LS-1
    // Because we're shifting left by 1 bit, we end up with a six bit value.
    // So we OR this with the 5th bit of the original lh_key which will give us
    // a circularly shifted key that is 6 bits long.
    // Because we only want a 5 bit long value, we just AND it with 31 like we
    // did above.
    let shifted_lh = ((lh_key << 1) | (lh_key >> (4))) & 0x1F;
    let shifted_rh = ((rh_key << 1) | (rh_key >> (4))) & 0x1F;

    println!("LS1 (LH): {:#0b}", shifted_lh);
    println!("LS1 (LH): {:#0b}", shifted_rh);

    // Combine shifted values to pass into P8.
    let shifted_combined = (shifted_lh << 5) | shifted_rh;
    println!("SC: {:#0b}", shifted_combined);

    let key_one = permute(shifted_combined, &P8, 8, 10);

    // LS-2 - Similar steps as above. We prep our key by splitting it into 2 5-bit
    // values. Then we shift them, combine them, and them run them through P8
    let lh_key = shifted_combined >> 5;
    let rh_key = shifted_combined & 0x1F;

    println!("LH: {:0b}", lh_key);
    println!("RH: {:0b}", rh_key);

    let shifted_lh = ((lh_key << 2) | (lh_key >> (3))) & 0x1F;
    let shifted_rh = ((rh_key << 2) | (rh_key >> (3))) & 0x1F;

    println!("LS2 (LH): {:#0b}", shifted_lh);
    println!("LS2 (LH): {:#0b}", shifted_rh);

    let shifted_combined = (shifted_lh << 5) | shifted_rh;

    let key_two = permute(shifted_combined, &P8, 8, 10);

    (key_one, key_two)
}

