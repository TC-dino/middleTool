/// Detects the SQL statement at the cursor position.
/// If text is selected, returns the selected text.
/// Otherwise, finds the statement containing the cursor (bounded by semicolons).
pub fn detect_statement_at_cursor(text: &str, cursor_offset: usize, selected_text: Option<&str>) -> String {
    // Priority 1: If text is selected, use it
    if let Some(selected) = selected_text {
        let trimmed = selected.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    // Priority 2: Detect statement at cursor position
    if text.is_empty() {
        return String::new();
    }

    // Clamp cursor offset to text length
    let cursor = cursor_offset.min(text.len());

    // Split text by semicolons into statements with their positions
    let mut statements = Vec::new();
    let mut start = 0;
    
    for (i, c) in text.char_indices() {
        if c == ';' {
            let stmt = text[start..i].trim();
            if !stmt.is_empty() {
                statements.push((start, i, stmt));
            }
            start = i + 1;
        }
    }
    
    // Don't forget the last statement (if no trailing semicolon)
    let last_stmt = text[start..].trim();
    if !last_stmt.is_empty() {
        statements.push((start, text.len(), last_stmt));
    }

    // Find which statement contains the cursor
    for (stmt_start, stmt_end, stmt) in &statements {
        // Cursor is in this statement if:
        // - It's after the start (or at the start)
        // - It's before the end (inclusive - cursor at semicolon belongs to this statement)
        if cursor >= *stmt_start && cursor <= *stmt_end {
            return stmt.to_string();
        }
    }

    // If cursor is at the end of the last statement, return it
    if let Some((_, _, stmt)) = statements.last() {
        if cursor >= text.len() {
            return stmt.to_string();
        }
    }

    // Fallback: return the last non-empty statement
    statements.last()
        .map(|(_, _, stmt)| stmt.to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_statement() {
        let text = "SELECT * FROM users";
        let result = detect_statement_at_cursor(text, 5, None);
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_multiple_statements_cursor_first() {
        let text = "SELECT * FROM users; SELECT * FROM orders;";
        let result = detect_statement_at_cursor(text, 5, None);
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_multiple_statements_cursor_second() {
        let text = "SELECT * FROM users; SELECT * FROM orders;";
        let result = detect_statement_at_cursor(text, 25, None);
        assert_eq!(result, "SELECT * FROM orders");
    }

    #[test]
    fn test_selected_text_priority() {
        let text = "SELECT * FROM users; SELECT * FROM orders;";
        let result = detect_statement_at_cursor(text, 5, Some("SELECT id FROM users"));
        assert_eq!(result, "SELECT id FROM users");
    }

    #[test]
    fn test_empty_text() {
        let result = detect_statement_at_cursor("", 0, None);
        assert_eq!(result, "");
    }

    #[test]
    fn test_cursor_at_end() {
        let text = "SELECT * FROM users";
        let result = detect_statement_at_cursor(text, 100, None);
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_cursor_at_semicolon() {
        let text = "SELECT * FROM users; SELECT * FROM orders";
        // Cursor at position 20 is right after the semicolon, so it belongs to the second statement
        let result = detect_statement_at_cursor(text, 20, None);
        assert_eq!(result, "SELECT * FROM orders");
        // Cursor at position 19 is at the semicolon, belongs to the first statement
        let result2 = detect_statement_at_cursor(text, 19, None);
        assert_eq!(result2, "SELECT * FROM users");
    }

    #[test]
    fn test_whitespace_handling() {
        let text = "  SELECT * FROM users  ;  SELECT * FROM orders  ";
        let result = detect_statement_at_cursor(text, 10, None);
        assert_eq!(result, "SELECT * FROM users");
    }
}
