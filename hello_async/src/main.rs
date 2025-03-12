use trpl::Html;

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;

    let text = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, text)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let first_url = &args[1];
        let second_url = &args[2];

        let title_1 = page_title(first_url);
        let title_2 = page_title(second_url);

        let (url, maybe_title) = match trpl::race(title_1, title_2).await {
            trpl::Either::Left(left) => left,
            trpl::Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("Title: {title}"),
            None => println!("No title returned :("),
        };
    })
}
