use chrono::Datelike;
use regex::Regex;

pub fn generate_slug(title: &str) -> String {
    // Remove special characters (keep only a-z, 0-9, spaces, and hyphens)
    let binding = title.to_lowercase();
    let re_special = Regex::new(r"[^a-z0-9\s\-]").unwrap();
    let step1 = re_special.replace_all(&binding, "");

    // Replace one or more spaces with single hyphen
    let re_spaces = Regex::new(r"\s+").unwrap();
    let step2 = re_spaces.replace_all(&step1, "-");

    // Replace multiple consecutive hyphens with single hyphen
    let re_hyphens = Regex::new(r"-+").unwrap();
    let step3 = re_hyphens.replace_all(&step2, "-");

    // Remove leading and trailing hyphens
    let re_trim = Regex::new(r"^-+|-+$").unwrap();
    re_trim.replace_all(&step3, "").trim().to_string()
}

pub fn generate_url(date: &chrono::DateTime<chrono::Utc>, slug: &str) -> String {
    format!("/{}/{}", format_date(date), slug)
}

pub fn format_date(date: &chrono::DateTime<chrono::Utc>) -> String {
    let year = date.year();
    let month = date.month();
    let day = date.day();

    format!("{:04}/{:02}/{:02}", year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_generate_slug_basic_conversion() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("My First Blog Post"), "my-first-blog-post");
        assert_eq!(generate_slug("Testing 123"), "testing-123");
    }

    #[test]
    fn test_generate_slug_special_characters_removal() {
        assert_eq!(generate_slug("Hello, World!"), "hello-world");
        assert_eq!(generate_slug("C++ Programming"), "c-programming");
        assert_eq!(generate_slug("React & TypeScript"), "react-typescript");
        assert_eq!(generate_slug("@username: A Story"), "username-a-story");
    }

    #[test]
    fn test_generate_slug_multiple_spaces_and_hyphens() {
        assert_eq!(generate_slug("Hello   World"), "hello-world");
        assert_eq!(generate_slug("Hello -- World"), "hello-world");
        assert_eq!(generate_slug("Hello  --  World"), "hello-world");
        assert_eq!(generate_slug("---Hello World---"), "hello-world");
    }

    #[test]
    fn test_generate_slug_edge_cases() {
        assert_eq!(generate_slug(""), "");
        assert_eq!(generate_slug("   "), "");
        assert_eq!(generate_slug("123"), "123");
        assert_eq!(generate_slug("---"), "");
    }

    #[test]
    fn test_generate_url_basic() {
        let date1 = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        assert_eq!(
            generate_url(&date1, "hello-world"),
            "/2025/01/15/hello-world"
        );

        let date2 = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        assert_eq!(
            generate_url(&date2, "year-end-post"),
            "/2025/12/31/year-end-post"
        );

        let date3 = Utc.with_ymd_and_hms(2024, 7, 4, 0, 0, 0).unwrap();
        assert_eq!(
            generate_url(&date3, "independence-day"),
            "/2024/07/04/independence-day"
        );
    }

    #[test]
    fn test_format_date_basic() {
        let date1 = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "2025/01/15");

        let date2 = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2025/12/31");

        let date3 = Utc.with_ymd_and_hms(2024, 7, 4, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date3), "2024/07/04");
    }

    #[test]
    fn test_format_date_single_digit_months_and_days() {
        let date1 = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "2025/01/01");

        let date2 = Utc.with_ymd_and_hms(2025, 1, 9, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2025/01/09");

        let date3 = Utc.with_ymd_and_hms(2025, 9, 1, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date3), "2025/09/01");

        let date4 = Utc.with_ymd_and_hms(2025, 2, 5, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date4), "2025/02/05");
    }

    #[test]
    fn test_format_date_leap_year() {
        let date1 = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "2024/02/29");

        let date2 = Utc.with_ymd_and_hms(2020, 2, 29, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2020/02/29");
    }

    #[test]
    fn test_format_date_edge_cases() {
        // New Year's Day
        let date1 = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "2025/01/01");

        // New Year's Eve
        let date2 = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2025/12/31");

        // Different centuries
        let date3 = Utc.with_ymd_and_hms(1999, 12, 31, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date3), "1999/12/31");

        let date4 = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date4), "2000/01/01");
    }

    #[test]
    fn test_format_date_various_years() {
        let date1 = Utc.with_ymd_and_hms(1990, 6, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "1990/06/15");

        let date2 = Utc.with_ymd_and_hms(2010, 3, 20, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2010/03/20");

        let date3 = Utc.with_ymd_and_hms(2030, 11, 8, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date3), "2030/11/08");
    }

    #[test]
    fn test_format_date_all_months() {
        let date1 = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date1), "2025/01/15");

        let date2 = Utc.with_ymd_and_hms(2025, 2, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date2), "2025/02/15");

        let date3 = Utc.with_ymd_and_hms(2025, 3, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date3), "2025/03/15");

        let date4 = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date4), "2025/04/15");

        let date5 = Utc.with_ymd_and_hms(2025, 5, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date5), "2025/05/15");

        let date6 = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date6), "2025/06/15");

        let date7 = Utc.with_ymd_and_hms(2025, 7, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date7), "2025/07/15");

        let date8 = Utc.with_ymd_and_hms(2025, 8, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date8), "2025/08/15");

        let date9 = Utc.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date9), "2025/09/15");

        let date10 = Utc.with_ymd_and_hms(2025, 10, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date10), "2025/10/15");

        let date11 = Utc.with_ymd_and_hms(2025, 11, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date11), "2025/11/15");

        let date12 = Utc.with_ymd_and_hms(2025, 12, 15, 0, 0, 0).unwrap();
        assert_eq!(format_date(&date12), "2025/12/15");
    }
}
