use reqwest::get;
use tokio::join;
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
	Ok(get(url).await?.text().await?)
}

pub async fn main(word: &str) -> Result<(String, String, String, String, String), reqwest::Error> {
	let urls: [&str; 5] = [
		&format!("https://dictionary.cambridge.org/dictionary/english/{}", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-verb-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-noun-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-adjective-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-adverb-for/{}.html", word)[..],
	];
	let (r1, r2, r3, r4, r5) = join!(
		fetch(urls[0]),
		fetch(urls[1]),
		fetch(urls[2]),
		fetch(urls[3]),
		fetch(urls[4]),
	);
	Ok((r1?, r2?, r3?, r4?, r5?))
}