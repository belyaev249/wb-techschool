#[tokio::main]
async fn main() {
    wget::process().await.unwrap();
}

mod wget {
    use clap::{Parser, arg};
    use reqwest::{Client, Response, Url};
    use::scraper::{Html, Selector};
    use std::io::Write;
    use std::path::PathBuf;
    use std::fs::{File, create_dir_all, remove_dir_all};
    use std::str::FromStr;
    use error::Error;
    use std::collections::{VecDeque, HashSet};

    #[derive(Parser)]
    struct Args {
        source_file_path: String,
        #[arg(default_value="output", short='o', long, help="")]
        output_dir: String
    }

    pub async fn process() -> Result<(), Error> {
        let client = Client::new();
        let args = Args::parse();

        remove_dir_all(args.output_dir.clone()).unwrap();

        let path_base = Url::from_str(&args.source_file_path).unwrap();
        let path_base_domain = path_base.domain().unwrap();
        let path_base = path_base.to_string();
        let path_base = path_base.trim_end_matches(path_base_domain);

        let mut paths_already_visited = HashSet::<String>::new();
        let mut paths_to_visit = VecDeque::from([args.source_file_path]);

        while let Some(path) = paths_to_visit.pop_front() {
            let mut absolute_path = Url::from_str(path_base)?;
            if let Ok(path) = Url::from_str(&path) {
                if let Some(domain) = path.domain() {
                    if domain != path_base_domain {
                        continue;
                    }
                }
                absolute_path = path;
            } else {
                absolute_path.set_path(&path);
            }
            
            println!("\n{: <15} {}", "Download from:", absolute_path.to_string());

            let domain_full_path = absolute_path.path().trim_matches('/');
            let domain_name = domain_full_path
                .split('/')
                .last()
                .filter(|x|
                    x.split('.').count() > 1
                )
                .unwrap_or("index.html");
            let domain_path = domain_full_path.trim_end_matches(&format!("/{domain_name}"));

            let domain_dir = PathBuf::from(&args.output_dir).join(domain_path);
            let domain_path = domain_dir.join(domain_name);
            create_dir_all(domain_dir)?;
            
            println!("{: <15} {}", "Save to:", domain_path.to_str().unwrap().to_string());

            let res = download(&client, absolute_path).await;
            let res = match res {
              Ok(r) => r,
              _ => continue,  
            };
            let mut bytes = res.bytes().await?;

            let mut file = File::create(domain_path)?;
            file.write_all(&mut bytes)?;

            let html_str = String::from_utf8(bytes.to_vec());
            let html_str = match html_str {
                Ok(s) => s,
                _ => continue,
            };

            let paths = get_paths(&html_str);

            for path in paths {
                if !paths_already_visited.contains(&path) {
                    paths_already_visited.insert(path.clone());
                    paths_to_visit.push_back(path);
                }
            }
        }

        Ok(())
    }

    fn get_paths(str: &str) -> HashSet::<String> {
        let html = Html::parse_document(str);
        let selector = Selector::parse("link, script, a, img").unwrap();

        let mut urls = HashSet::<String>::new();
        for element in html.select(&selector) {
            if let Some(url) = element.attr("href").or(element.attr("src")) {
                urls.insert(url.to_string());
            }
        }
        return urls;
    }

    async fn download(client: &Client, url: Url) -> Result<Response, Error> {
        let r = client.get(url).send().await?;
        Ok(r)
    }

    mod error {
        use std::fmt::Debug;

        pub enum Error {
            Unknown(String)
        }

        impl<T> From<T> for Error where T: ToString {
            fn from(value: T) -> Error {
                Error::Unknown(value.to_string())
            }
        }

        impl Debug for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let msg = match self {
                    Error::Unknown(msg) => msg
                };
                write!(f, "{msg}")?;
                Ok(())
            }
        }
    }
}