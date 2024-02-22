#[cfg(test)]
mod test_zrange_vec_to_tuple_vec {
    use crate::database::zrange_vec_to_tuple_vec;

    #[test]
    fn should_parse_the_score_and_return_a_vec_of_tuples() {
        let v = vec![
            "member1".to_string(),
            "123".to_string(),
            "member2".to_string(),
            "456".to_string(),
        ];
        let result = zrange_vec_to_tuple_vec(v);
        assert_eq!(
            result,
            Ok(vec![
                (123, "member1".to_string()),
                (456, "member2".to_string())
            ])
        );
    }
}
