#![cfg(test)]
#![allow(dead_code)]
extern crate rand;

use super::*;
use super::Error;
use super::shard_utils;
use self::rand::{thread_rng, Rng};

#[macro_export]
macro_rules! convert_2D_slices {
    (
        $slice:ident =into_vec=> $dst_type:ty
    ) => {
        convert_2D_slices!($slice =into=> Vec<$dst_type>,
                           Vec::with_capacity)
    };
    (
        $slice:ident =to_vec=> $dst_type:ty
    ) => {
        convert_2D_slices!($slice =to=> Vec<$dst_type>,
                           Vec::with_capacity)
    };
    (
        $slice:ident =to_mut_vec=> $dst_type:ty
    ) => {
        convert_2D_slices!($slice =to_mut=> Vec<$dst_type>,
                           Vec::with_capacity)
    };
    (
        $slice:ident =into=> $dst_type:ty, $with_capacity:path
    ) => {{
        let mut result : $dst_type =
            $with_capacity($slice.len());
        for i in $slice.into_iter() {
            result.push(i);
        }
        result
    }};
    (
        $slice:ident =to=> $dst_type:ty, $with_capacity:path
    ) => {{
        let mut result : $dst_type =
            $with_capacity($slice.len());
        for i in $slice.iter() {
            result.push(i);
        }
        result
    }};
    (
        $slice:ident =to_mut=> $dst_type:ty, $with_capacity:path
    ) => {{
        let mut result : $dst_type =
            $with_capacity($slice.len());
        for i in $slice.iter_mut() {
            result.push(i);
        }
        result
    }}
}

macro_rules! shards {
    (
        $( [ $( $x:expr ),* ] ),*
    ) => {{
        vec![ $( vec![ $( $x as u8),* ].into_boxed_slice() ),* ]
    }}
}

macro_rules! make_random_shards {
    ($per_shard:expr, $size:expr) => {{
        let mut shards = Vec::with_capacity(13);
        for _ in 0..$size {
            shards.push(make_blank_shard($per_shard));
        }

        for s in shards.iter_mut() {
            fill_random(s);
        }

        shards
    }}
}

fn assert_eq_shards(s1 : &Vec<Shard>, s2 : &Vec<Shard>) {
    assert_eq!(s1.len(), s2.len());
    for i in 0..s1.len() {
        assert_eq!(s1[i],
                   s2[i]);
    }
}

/*
fn is_increasing_and_contains_data_row(indices : &Vec<usize>) -> bool {
    let cols = indices.len();
    for i in 0..cols-1 {
        if indices[i] >= indices[i+1] {
            return false
        }
    }
    return indices[0] < cols
}

fn increment_indices(indices : &mut Vec<usize>,
                     index_bound : usize) -> bool {
    for i in (0..indices.len()).rev() {
        indices[i] += 1;
        if indices[i] < index_bound {
            break;
        }

        if i == 0 {
            return false
        }

        indices[i] = 0
    }

    return true
}

fn increment_indices_until_increasing_and_contains_data_row(indices : &mut Vec<usize>, max_index : usize) -> bool {
    loop {
        let valid = increment_indices(indices, max_index);
        if !valid {
            return false
        }

        if is_increasing_and_contains_data_row(indices) {
            return true
        }
    }
}

fn find_singular_sub_matrix(m : Matrix) -> Option<Matrix> {
    let rows = m.row_count();
    let cols = m.col_count();
    let mut row_indices = Vec::with_capacity(cols);
    while increment_indices_until_increasing_and_contains_data_row(&mut row_indices, rows) {
        let mut sub_matrix = Matrix::new(cols, cols);
        for i in 0..row_indices.len() {
            let r = row_indices[i];
            for c in 0..cols {
                sub_matrix.set(i, c, m.get(r, c));
            }
        }

        match sub_matrix.invert() {
            Err(matrix::Error::SingularMatrix) => return Some(sub_matrix),
            whatever => whatever.unwrap()
        };
    }
    None
}*/

fn fill_random(arr : &mut Shard) {
    for a in arr.iter_mut() {
        *a = rand::random::<u8>();
    }
}

/*fn assert_eq_shards_with_range(shards1    : &Vec<Shard>,
                               shards2    : &Vec<Shard>,
                               offset     : usize,
                               byte_count : usize) {
    for s in 0..shards1.len() {
        let slice1 = &shards1[s][offset..offset + byte_count];
        let slice2 = &shards2[s][offset..offset + byte_count];
        assert_eq!(slice1, slice2);
    }
}*/

#[test]
#[should_panic]
fn test_no_data_shards() {
    ReedSolomon::new(0, 1); }

#[test]
#[should_panic]
fn test_no_parity_shards() {
    ReedSolomon::new(1, 0); }

#[test]
fn test_shard_count() {
    let mut rng = thread_rng();
    for _ in 0..10 {
        let data_shard_count   = rng.gen_range(1, 128);
        let parity_shard_count = rng.gen_range(1, 128);

        let total_shard_count = data_shard_count + parity_shard_count;

        let r = ReedSolomon::new(data_shard_count, parity_shard_count);

        assert_eq!(data_shard_count,   r.data_shard_count());
        assert_eq!(parity_shard_count, r.parity_shard_count());
        assert_eq!(total_shard_count,  r.total_shard_count());
    }
}

/*#[test]
#[should_panic]
fn test_calc_byte_count_byte_count_is_zero_case1() {
    let shards = make_random_shards!(1_000, 1);

    shard_utils::helper::calc_byte_count(&shards,
                                         Some(0)); }*/

/*#[test]
#[should_panic]
fn test_calc_byte_count_byte_count_is_zero_case2() {
    let shards = make_random_shards!(1_000, 0);

    shard_utils::helper::calc_byte_count(&shards,
                                         None); }*/

/*#[test]
#[should_panic]
fn test_calc_byte_count_option_shards_byte_count_is_zero_case1() {
    let shards = make_random_shards!(1_000, 1);
    let option_shards = shards_into_option_shards(shards);

    shard_utils::helper::calc_byte_count_option_shards(&option_shards,
                                                       Some(0)); }*/

/*#[test]
#[should_panic]
fn test_calc_byte_count_option_shards_byte_count_is_zero_case2() {
    let shards = make_random_shards!(1_000, 0);
    let option_shards = shards_into_option_shards(shards);

    shard_utils::helper::calc_byte_count_option_shards(&option_shards,
                                                       None); }*/

/*#[test]
#[should_panic]
fn test_calc_byte_count_option_shards_no_shards_present() {
    let shards = make_random_shards!(1_000, 2);

    let mut option_shards = shards_into_option_shards(shards);

    option_shards[0] = None;
    option_shards[1] = None;

    shard_utils::helper::calc_byte_count_option_shards(&option_shards,
                                                       None); }*/

#[test]
fn test_shards_into_option_shards_into_shards() {
    for _ in 0..100 {
        let shards = make_random_shards!(1_000, 10);
        let expect = shards.clone();
        let inter  =
            shard_utils::shards_into_option_shards(shards);
        let result =
            shard_utils::option_shards_into_shards(inter);

        assert_eq_shards(&expect, &result);
    }
}

#[test]
fn test_shards_to_option_shards_to_shards() {
    for _ in 0..100 {
        let shards = make_random_shards!(1_000, 10);
        let expect = shards.clone();
        let option_shards =
            shard_utils::shards_to_option_shards(&shards);
        let result        =
            shard_utils::option_shards_to_shards(&option_shards,
                                                 None, None);

        assert_eq_shards(&expect, &result);
    }
}

#[test]
#[should_panic]
fn test_option_shards_to_shards_missing_shards_case1() {
    let shards = make_random_shards!(1_000, 10);
    let mut option_shards = shards_into_option_shards(shards);

    option_shards[0] = None;

    shard_utils::option_shards_to_shards(&option_shards, None, None);
}

#[test]
fn test_option_shards_to_shards_missing_shards_case2() {
    let shards = make_random_shards!(1_000, 10);
    let mut option_shards = shards_into_option_shards(shards);

    option_shards[0] = None;
    option_shards[9] = None;

    shard_utils::option_shards_to_shards(&option_shards, Some(1), Some(8));
}

#[test]
#[should_panic]
fn test_option_shards_into_missing_shards() {
    let shards = make_random_shards!(1_000, 10);
    let mut option_shards = shards_into_option_shards(shards);

    option_shards[2] = None;

    shard_utils::option_shards_into_shards(option_shards);
}

#[test]
#[should_panic]
fn test_option_shards_to_shards_too_few_shards() {
    let shards = make_random_shards!(1_000, 10);
    let option_shards = shards_into_option_shards(shards);

    shard_utils::option_shards_to_shards(&option_shards,
                            None,
                            Some(11));
}

#[test]
fn test_reedsolomon_clone() {
    let r1 = ReedSolomon::new(10, 3);
    let r2 = r1.clone();

    assert_eq!(r1, r2);
}

#[test]
#[should_panic]
fn test_reedsolomon_too_many_shards() {
    ReedSolomon::new(256, 1); }

#[test]
fn test_encoding() {
    let per_shard = 50_000;

    let r = ReedSolomon::new(10, 3);

    let mut shards = make_random_shards!(per_shard, 13);

    r.encode_shards(&mut shards).unwrap();
    assert!(r.verify_shards(&shards).unwrap());

    assert_eq!(Error::TooFewShards,
               r.encode_shards(&mut shards[0..1]).unwrap_err());

    let mut bad_shards = make_random_shards!(per_shard, 13);
    bad_shards[0] = vec![0 as u8].into_boxed_slice();
    assert_eq!(Error::IncorrectShardSize,
               r.encode_shards(&mut bad_shards).unwrap_err());
}

#[test]
fn test_reconstruct_shards() {
    let per_shard = 100_000;

    let r = ReedSolomon::new(8, 5);

    let mut shards = make_random_shards!(per_shard, 13);

    r.encode_shards(&mut shards).unwrap();

    let master_copy = shards.clone();

    let mut shards = shards_to_option_shards(&shards);

    // Try to decode with all shards present
    r.reconstruct_shards(&mut shards).unwrap();
    {
        let shards = option_shards_to_shards(&shards, None, None);
        assert!(r.verify_shards(&shards).unwrap());
        assert_eq_shards(&shards, &master_copy);
    }

    // Try to decode with 10 shards
    shards[0] = None;
    shards[2] = None;
    //shards[4] = None;
    r.reconstruct_shards(&mut shards).unwrap();
    {
        let shards = option_shards_to_shards(&shards, None, None);
        assert!(r.verify_shards(&shards).unwrap());
        assert_eq_shards(&shards, &master_copy);
    }

    // Try to deocde with 6 data and 4 parity shards
    shards[0] = None;
    shards[2] = None;
    shards[12] = None;
    r.reconstruct_shards(&mut shards).unwrap();
    {
        let shards = option_shards_to_shards(&shards, None, None);
        assert!(r.verify_shards(&shards).unwrap());
        assert_eq_shards(&shards, &master_copy);
    }

    // Try to reconstruct data only
    shards[0] = None;
    shards[1] = None;
    shards[12] = None;
    r.reconstruct_data_shards(&mut shards).unwrap();
    {
        let shards = option_shards_to_shards(&shards, None, None);
        let blank_shard = make_blank_shard(per_shard);
        assert_eq!(master_copy[0], shards[0]);
        assert_eq!(master_copy[1], shards[1]);
        assert_eq!(blank_shard, shards[12]);
    }

    // Try to decode with 7 data and 1 parity shards
    shards[0] = None;
    shards[1] = None;
    shards[9] = None;
    shards[10] = None;
    shards[11] = None;
    shards[12] = None;
    assert_eq!(r.reconstruct_shards(&mut shards).unwrap_err(),
               Error::TooFewShardsPresent);
}

#[test]
fn test_reconstruct() {
    let r = ReedSolomon::new(2, 2);

    let mut shards : [[u8; 3]; 4] = [[0, 1, 2],
                                     [3, 4, 5],
                                     [200, 201, 203],
                                     [100, 101, 102]];

    {
        let mut shard_refs : Vec<&mut [u8]> =
            Vec::with_capacity(3);

        for shard in shards.iter_mut() {
            shard_refs.push(shard);
        }

        r.encode(&mut shard_refs).unwrap();
    }

    {
        let mut shard_refs =
            convert_2D_slices!(shards =to_mut_vec=> &mut [u8]);

        shard_refs[0][0] = 101;
        shard_refs[0][1] = 102;
        shard_refs[0][2] = 103;

        let shards_present = [false, true, true, true];

        r.reconstruct(&mut shard_refs, &shards_present).unwrap();
    }

    let expect : [[u8; 3]; 4] = [[0, 1, 2],
                                 [3, 4, 5],
                                 [6, 11, 12],
                                 [5, 14, 11]];
    assert_eq!(expect, shards);

    {
        let mut shard_refs =
            convert_2D_slices!(shards =to_mut_vec=> &mut [u8]);

        shard_refs[0][0] = 201;
        shard_refs[0][1] = 202;
        shard_refs[0][2] = 203;

        shard_refs[2][0] = 101;
        shard_refs[2][1] = 102;
        shard_refs[2][2] = 103;

        let shards_present = [false, true, false, true];

        r.reconstruct_data(&mut shard_refs,
                           &shards_present).unwrap();
    }

    let expect : [[u8; 3]; 4] = [[0, 1, 2],
                                 [3, 4, 5],
                                 [101, 102, 103],
                                 [5, 14, 11]];
    assert_eq!(expect, shards);

    {
        let mut shard_refs =
            convert_2D_slices!(shards =to_mut_vec=> &mut [u8]);

        shard_refs[2][0] = 101;
        shard_refs[2][1] = 102;
        shard_refs[2][2] = 103;

        shard_refs[3][0] = 201;
        shard_refs[3][1] = 202;
        shard_refs[3][2] = 203;

        let shards_present = [true, true, false, false];

        r.reconstruct_data(&mut shard_refs,
                           &shards_present).unwrap();
    }

    let expect : [[u8; 3]; 4] = [[0, 1, 2],
                                 [3, 4, 5],
                                 [101, 102, 103],
                                 [201, 202, 203]];
    assert_eq!(expect, shards);
}

/*
#[test]
fn test_is_parity_correct() {
    let per_shard = 33_333;

    let r = ReedSolomon::new(10, 4);

    let mut shards = make_random_shards!(per_shard, 14);

    r.encode_parity(&mut shards, None, None);
    assert!(r.is_parity_correct(&shards, None, None));

    // corrupt shards
    fill_random(&mut shards[5]);
    assert!(!r.is_parity_correct(&shards, None, None));

    // Re-encode
    r.encode_parity(&mut shards, None, None);
    fill_random(&mut shards[1]);
    assert!(!r.is_parity_correct(&shards, None, None));
}

#[test]
fn test_is_parity_correct_with_range() {
    let per_shard = 33_333;

    let offset = 7;
    let byte_count = 100;
    let op_offset = Some(offset);
    let op_byte_count = Some(byte_count);

    let r = ReedSolomon::new(10, 4);

    let mut shards = make_random_shards!(per_shard, 14);

    r.encode_parity(&mut shards, op_offset, op_byte_count);
    assert!(r.is_parity_correct(&shards, op_offset, op_byte_count));

    // corrupt shards
    fill_random(&mut shards[5]);
    assert!(!r.is_parity_correct(&shards, op_offset, op_byte_count));

    // Re-encode
    r.encode_parity(&mut shards, op_offset, op_byte_count);
    fill_random(&mut shards[1]);
    assert!(!r.is_parity_correct(&shards, op_offset, op_byte_count));
}
*/

#[test]
fn test_one_encode() {
    let r = ReedSolomon::new(5, 5);

    let mut shards = shards!([0, 1],
                             [4, 5],
                             [2, 3],
                             [6, 7],
                             [8, 9],
                             [0, 0],
                             [0, 0],
                             [0, 0],
                             [0, 0],
                             [0, 0]);

    r.encode_shards(&mut shards).unwrap();
    { assert_eq!(shards[5][0], 12);
      assert_eq!(shards[5][1], 13); }
    { assert_eq!(shards[6][0], 10);
      assert_eq!(shards[6][1], 11); }
    { assert_eq!(shards[7][0], 14);
      assert_eq!(shards[7][1], 15); }
    { assert_eq!(shards[8][0], 90);
      assert_eq!(shards[8][1], 91); }
    { assert_eq!(shards[9][0], 94);
      assert_eq!(shards[9][1], 95); }

    assert!(r.verify_shards(&shards).unwrap());

    shards[8][0] += 1;
    assert!(!r.verify_shards(&shards).unwrap());
}

#[test]
fn test_verify_too_few_shards() {
    let r = ReedSolomon::new(3, 2);

    let shards = make_random_shards!(10, 4);

    assert_eq!(Error::TooFewShards, r.verify_shards(&shards).unwrap_err());
}

#[test]
fn test_slices_or_shards_count_check() {
    let r = ReedSolomon::new(3, 2);

    {
        let mut shards = make_random_shards!(10, 4);

        assert_eq!(Error::TooFewShards, r.encode_shards(&mut shards).unwrap_err());
        assert_eq!(Error::TooFewShards, r.verify_shards(&shards).unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::TooFewShards, r.reconstruct_shards(&mut option_shards).unwrap_err());
    }
    {
        let mut shards = make_random_shards!(10, 6);

        assert_eq!(Error::TooManyShards, r.encode_shards(&mut shards).unwrap_err());
        assert_eq!(Error::TooManyShards, r.verify_shards(&shards).unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::TooManyShards, r.reconstruct_shards(&mut option_shards).unwrap_err());
    }
}

#[test]
fn test_check_slices_or_shards_size() {
    let r = ReedSolomon::new(2, 2);

    {
        let mut shards = shards!([0, 0, 0],
                                 [0, 1],
                                 [1, 2, 3],
                                 [0, 0, 0]);

        assert_eq!(Error::IncorrectShardSize,
                   r.encode_shards(&mut shards)
                   .unwrap_err());
        assert_eq!(Error::IncorrectShardSize,
                   r.verify_shards(&shards)
                   .unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::IncorrectShardSize,
                   r.reconstruct_shards(&mut option_shards)
                   .unwrap_err());
    }
    {
        let mut shards = shards!([0, 1],
                                 [0, 1],
                                 [1, 2, 3],
                                 [0, 0, 0]);

        assert_eq!(Error::IncorrectShardSize,
                   r.encode_shards(&mut shards)
                   .unwrap_err());
        assert_eq!(Error::IncorrectShardSize,
                   r.verify_shards(&shards)
                   .unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::IncorrectShardSize,
                   r.reconstruct_shards(&mut option_shards)
                   .unwrap_err());
    }
    {
        let mut shards = shards!([0, 1],
                                 [0, 1, 4],
                                 [1, 2, 3],
                                 [0, 0, 0]);

        assert_eq!(Error::IncorrectShardSize,
                   r.encode_shards(&mut shards)
                   .unwrap_err());
        assert_eq!(Error::IncorrectShardSize,
                   r.verify_shards(&shards)
                   .unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::IncorrectShardSize,
                   r.reconstruct_shards(&mut option_shards)
                   .unwrap_err());
    }
    {
        let mut shards = shards!([],
                                 [0, 1, 3],
                                 [1, 2, 3],
                                 [0, 0, 0]);

        assert_eq!(Error::EmptyShard,
                   r.encode_shards(&mut shards)
                   .unwrap_err());
        assert_eq!(Error::EmptyShard,
                   r.verify_shards(&shards)
                   .unwrap_err());

        let mut option_shards = shards_to_option_shards(&shards);

        assert_eq!(Error::EmptyShard,
                   r.reconstruct_shards(&mut option_shards)
                   .unwrap_err());
    }
    {
        let mut option_shards = vec![None, None, None, None];

        assert_eq!(Error::TooFewShardsPresent,
                   r.reconstruct_shards(&mut option_shards)
                   .unwrap_err());
    }
}
