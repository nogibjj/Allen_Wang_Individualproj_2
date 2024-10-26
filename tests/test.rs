use assert_cmd::Command;

#[test]
fn test_transform() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("transform")
        .arg("drink.db")
        .arg("https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv")
        .assert()
        .success(); // Assert that the command executed successfully
}

#[test]
fn test_create_row() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("create")
        .arg("USB") // country
        .arg("10") // beer_servings
        .arg("20") // spirit_servings
        .arg("30") // wine_servings
        .arg("10.7") // total_litres_of_pure_alcohol
        .assert()
        .success(); // Assert that the command executed successfully
}

#[test]
fn test_read_all() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("read").assert().success(); // Assert that the command executed successfully
}

#[test]
fn test_update_row() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("update")
        .arg("USA") // country
        .arg("15") // new beer_servings
        .assert()
        .success(); // Assert that the command executed successfully
}

#[test]
fn test_delete_row() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("delete").arg("Canada").assert().success(); // Assert that the command executed successfully
}

#[test]
fn test_general() {
    let mut cmd = Command::cargo_bin("allen_crud").unwrap(); // Replace with your binary name
    cmd.arg("general")
        .arg("UPDATE drink SET beer_servings = 100 WHERE country = USB")
        .assert()
        .success(); // Assert that the command executed successfully
}
