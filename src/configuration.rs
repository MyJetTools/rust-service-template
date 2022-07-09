use serde::{de::DeserializeOwned, Serialize, Deserialize};
use tokio::{fs::File, io::AsyncReadExt};

pub struct SettingsReader {}

impl SettingsReader {
    pub async fn read_settings<T>() -> Result<T, ()> where T: DeserializeOwned {
        if let Ok(result) = read_from_url::<T>().await {
            return Ok(result);
        }

        if let Ok(result) = read_from_file::<T>().await {
            return Ok(result);
        } else {
            return Err(());
        }
    }
}

pub async fn read_from_url<T>() -> Result<T, ()> where T: DeserializeOwned {
    let settings_url = std::env::var("SETTINGS_URL".to_string());

    match settings_url {
        Ok(res) => {
            let client = reqwest::Client::new();
            let response_result = client.get(res).send().await;
            match response_result {
                Ok(response) => {
                    let bytes = response.bytes().await.unwrap();
                    let data = bytes.to_vec();
                    println!("{:?}", data);
                    let result: T = serde_yaml::from_slice(&data).unwrap();
                    return Ok(result);
                }
                Err(_) => println!("Settings url is not set!"),
            }
        }
        Err(_) => println!("Settings url is not set!"),
    }

    read_from_file::<T>().await
}

async fn read_from_file<T>() -> Result<T, ()> where T: DeserializeOwned {
    let filename = get_settings_filename();

    println!("Reading settings file {}", filename);

    let file = File::open(&filename).await;

    if let Err(err) = file {
        panic!(
            "Can not open settings file: {}. The reason is: {:?}",
            filename, err
        );
    }

    let mut file = file.unwrap();

    let mut file_content: Vec<u8> = Vec::new();

    loop {
        let res = file.read_buf(&mut file_content).await.unwrap();

        if res == 0 {
            break;
        }
    }

    let result: T = serde_yaml::from_slice(&file_content).unwrap();

    Ok(result)
}

#[cfg(target_os = "windows")]
fn get_settings_filename() -> String {
    let home_path = env!("HOME");
    let filename = format!("{}\\{}", home_path, ".settings");
    filename
}

#[cfg(not(target_os = "windows"))]
fn get_settings_filename() -> String {
    let home_path = env!("HOME");
    let filename = format!("{}/{}", home_path, ".settings");
    filename
}

#[cfg(test)]
mod tests {
    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    use super::SettingsReader;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SettingsModel {
        #[serde(rename = "RustServiceTemplateTest")]
        pub inner: SettingsModelInner,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SettingsModelInner {
        #[serde(rename = "ZipkinUrl")]
        pub zipkin_url: String,

        #[serde(rename = "SeqServiceUrl")]
        pub seq_service_url: String,
    }

    #[tokio::test]
    async fn test_something_async() {
        let settings = SettingsReader::read_settings::<SettingsModel>().await.unwrap();
        println!("{:?}", settings);
    }
}
