#[cfg(test)]
mod tests {
    use typikon::cli::commands::handle_watch_command;

    #[test]
    fn test_live_server() {
        assert_eq!(handle_watch_command(), ());
    }
}
