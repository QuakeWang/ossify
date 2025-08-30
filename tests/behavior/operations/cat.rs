use crate::*;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use storify::error::Result;
use storify::storage::StorageClient;

pub fn tests(client: &StorageClient, tests: &mut Vec<Trial>) {
    tests.extend(async_trials!(client, test_cat_small_file));
}

async fn test_cat_small_file(_client: StorageClient) -> Result<()> {
    let (path, content, _) = TEST_FIXTURE.new_file(_client.operator());
    _client.operator().write(&path, content).await?;

    storify_cmd()
        .arg("cat")
        .arg(&path)
        .assert()
        .success()
        .stdout("File cat success");
    Ok(())
}
