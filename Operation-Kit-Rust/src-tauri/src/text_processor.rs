use crate::error::{AppError, AppResult};

pub fn convert_format(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::FormatError("输入不能为空".to_string()));
    }
    
    let uses_newlines = input.matches('\n').count() > input.matches(',').count();
    let items: Vec<&str> = input
        .split(|c| c == '\n' || c == ',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let delimiter = if uses_newlines { "," } else { "\n" };
    Ok(items.join(delimiter))
}

pub fn replace_chinese_commas(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::FormatError("输入不能为空".to_string()));
    }
    Ok(input.replace('，', ","))
}

pub fn add_quotes(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::FormatError("输入不能为空".to_string()));
    }
    
    let uses_newlines = input.matches('\n').count() > input.matches(',').count();
    let items: Vec<&str> = input
        .split(|c| c == '\n' || c == ',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let delimiter = if uses_newlines { "\n" } else { "," };
    let processed: Vec<String> = items
        .into_iter()
        .map(|item| {
            if !item.starts_with('\'') && !item.ends_with('\'') {
                format!("'{}'", item)
            } else {
                item.to_string()
            }
        })
        .collect();

    Ok(processed.join(delimiter))
}

pub fn remove_quotes(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::FormatError("输入不能为空".to_string()));
    }
    
    let uses_newlines = input.matches('\n').count() > input.matches(',').count();
    let items: Vec<&str> = input
        .split(|c| c == '\n' || c == ',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let delimiter = if uses_newlines { "\n" } else { "," };
    let processed: Vec<String> = items
        .into_iter()
        .map(|item| {
            if item.starts_with('\'') && item.ends_with('\'') {
                item[1..item.len()-1].to_string()
            } else {
                item.to_string()
            }
        })
        .collect();

    Ok(processed.join(delimiter))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_format() {
        let input = "1,2,3";
        let result = convert_format(input).unwrap();
        assert_eq!(result, "1\n2\n3");

        let input = "1\n2\n3";
        let result = convert_format(input).unwrap();
        assert_eq!(result, "1,2,3");
    }

    #[test]
    fn test_replace_chinese_commas() {
        let input = "1，2，3";
        let result = replace_chinese_commas(input).unwrap();
        assert_eq!(result, "1,2,3");
    }

    #[test]
    fn test_add_remove_quotes() {
        let input = "1,2,3";
        let with_quotes = add_quotes(input).unwrap();
        assert_eq!(with_quotes, "'1','2','3'");

        let result = remove_quotes(&with_quotes).unwrap();
        assert_eq!(result, "1,2,3");
    }
} 