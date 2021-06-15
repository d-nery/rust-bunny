pub fn get_command_and_args_from_query_string(query_string: &str) -> (&str, Option<Vec<&str>>) {
    let split: Vec<&str> = query_string.split_whitespace().collect();

    if split.len() > 1 {
        return (split[0], Some(split[1..].to_vec()));
    }

    (&query_string, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let (actual_cmd, actual_args) = get_command_and_args_from_query_string("tw");
        let expected_cmd = "tw";
        assert_eq!(actual_cmd, expected_cmd);
        assert_eq!(actual_args, None);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let (actual_cmd, actual_args) = get_command_and_args_from_query_string("tw @test");
        let expected_cmd = "tw";
        let unwraped_args = actual_args.unwrap();

        assert_eq!(actual_cmd, expected_cmd);
        assert_eq!(unwraped_args.len(), 1);
        assert_eq!(unwraped_args[0], "@test");
    }
}
