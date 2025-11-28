fn url_1(page: u32) -> anyhow::Result<String> {
    let base_url =
        "https://pluto0x0.github.io/X_based_china";
    let url = if page == 1 {
        format!("{}/", base_url)
    } else {
        format!("{}/page{}.html", base_url, page)
    };
    Ok(url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_1() {}

    #[test]
    fn url() {
        let url = url_1(1).unwrap();
        assert_eq!(
            url,
            "https://pluto0x0.github.io/X_based_china/"
        );

        let url = url_1(3).unwrap();
        assert_eq!(
            url,
            "https://pluto0x0.github.io/X_based_china/page3.html"
        );
    }
}
