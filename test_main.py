import subprocess
import os


def test_transform():
    result = subprocess.run(
        [
            "python",
            "main.py",
            "transform",
            "drink.db",
            "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_create_row():
    result = subprocess.run(
        [
            "python",
            "main.py",
            "create",
            "USB",  # country
            "10",  # beer_servings
            "20",  # spirit_servings
            "30",  # wine_servings
            "10.7",  # total_litres_of_pure_alcohol
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_read_all():
    result = subprocess.run(
        ["python", "main.py", "read"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_update_row():
    result = subprocess.run(
        [
            "python",
            "main.py",
            "update",
            "USA",  # country
            "15",  # new beer_servings
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_delete_row():
    result = subprocess.run(
        ["python", "main.py", "delete", "Canada"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_general():
    result = subprocess.run(
        [
            "python",
            "main.py",
            "general",
            "UPDATE drink SET beer_servings = 100 WHERE country = USB",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":

    test_transform()
    test_create_row()
    test_read_all()
    test_update_row()
    test_delete_row()
    test_general()
